use std::collections::{HashMap, HashSet};

use crate::{lexer::Token, parser::{Declaration, Expression, RootAST, Statement, Type}};

#[derive(Debug)]
pub enum Symbol {
    Variable {
        kind: Type,
    },
    Enum {
        mods: HashSet<String>,
        variants: Vec<String>,
    },
    Struct {
        fields: HashMap<String, Type>,
    },
    Type {
        kind: Type,
    }
}

type Environment = Vec<HashMap<String, Symbol>>;
pub struct Analyzer {
    env: Environment,

    pub warn_count: usize,
    pub err_count: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),

            warn_count: 0,
            err_count: 0,
        }
    }

    fn warning(&mut self, msg: &str) {
        self.warn_count += 1;
        eprintln!("\x1b[0;33mwarning\x1b[0;0m: {msg}");
    }
    fn error(&mut self, msg: &str) {
        self.err_count += 1;
        eprintln!("\x1b[0;31merror\x1b[0;0m: {msg}");
    }

    fn add_table(&mut self) {
        self.env.push(HashMap::new());
    }
    fn pop_table(&mut self) {
        self.env.pop();
    }
    fn add_entry(&mut self, name: String, symbol: Symbol) {
        if self.env.is_empty() {
            self.add_table();
        }

        if self.env.last_mut().unwrap().insert(name.clone(), symbol).is_some() {
            eprintln!("\x1b[0;31merror\x1b[0;0m: redefinition of {}", name);
        }
    }
    fn get_entry(&mut self, name: &String) -> Option<&Symbol> {
        self.env.iter().rev().find_map(|table| table.get(name))
    }

    fn comptime_eval(expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Binary { lhs, rhs, op } => {
                let eval_logic = |lhs: &Expression, rhs: &Expression, op: fn(bool, bool) -> bool| -> Option<Expression> {
                    let lhs_eval = {
                        let Expression::Integer(val, _) = Self::comptime_eval(lhs)? else { unreachable!(); };
                        val != 0
                    };
                    let rhs_eval = {
                        let Expression::Integer(val, _) = Self::comptime_eval(rhs)? else { unreachable!(); };
                        val != 0
                    };

                    Some(Expression::Integer(op(lhs_eval, rhs_eval) as i128, "i32".into()))
                };
                let eval_math = |lhs: &Expression, rhs: &Expression, op: fn(i128, i128) -> i128| -> Option<Expression> {
                    let Expression::Integer(lhs_eval, _) = Self::comptime_eval(lhs)? else { unreachable!(); };
                    let Expression::Integer(rhs_eval, _) = Self::comptime_eval(rhs)? else { unreachable!(); };

                    Some(Expression::Integer(op(lhs_eval, rhs_eval), "i32".into()))
                };
                match op {
                    Token::Equals => Self::comptime_eval(rhs),
                    Token::LogOr => eval_logic(lhs, rhs, |l, r| l || r),
                    Token::LogAnd => eval_logic(lhs, rhs, |l, r| l && r),
                    Token::DoubleEquals => eval_logic(lhs, rhs, |l, r| l == r),
                    Token::NotEquals => eval_logic(lhs, rhs, |l, r| l != r),
                    Token::LessThan => eval_logic(lhs, rhs, |l, r| l < r),
                    Token::MoreThan => eval_logic(lhs, rhs, |l, r| l > r),
                    Token::LessThanEq => eval_logic(lhs, rhs, |l, r| l <= r),
                    Token::MoreThanEq => eval_logic(lhs, rhs, |l, r| l >= r),

                    Token::BitOr => eval_math(lhs, rhs, |l, r| l | r),
                    Token::BitXor => eval_math(lhs, rhs, |l, r| l ^ r),
                    Token::Ampersand => eval_math(lhs, rhs, |l, r| l & r),
                    Token::LShift => eval_math(lhs, rhs, |l, r| l << r),
                    Token::RShift => eval_math(lhs, rhs, |l, r| l >> r),
                    Token::Plus => eval_math(lhs, rhs, |l, r| l + r),
                    Token::Minus => eval_math(lhs, rhs, |l, r| l - r),
                    Token::Star => eval_math(lhs, rhs, |l, r| l * r),
                    Token::Slash => eval_math(lhs, rhs, |l, r| l / r),
                    Token::Percent => eval_math(lhs, rhs, |l, r| l % r),

                    _ => unreachable!("parser should have dealt with an unknown binary operator way before here i think"),
                }
            }
            Expression::Unary { child, op } => {
                match op {
                    Token::LogNot => {
                        let (eval, kind) = {
                            let Expression::Integer(v, k) = Self::comptime_eval(child)? else { unreachable!(); };
                            (v != 0, k)
                        };
                        Some(Expression::Integer((!eval) as i128, kind))
                    }
                    Token::Minus => {
                        let Expression::Integer(eval, kind) = Self::comptime_eval(child)? else { unreachable!(); };
                        Some(Expression::Integer((-eval) as i128, kind))
                    }
                    Token::BitNeg => {
                        let Expression::Integer(eval, kind) = Self::comptime_eval(child)? else { unreachable!(); };
                        Some(Expression::Integer(!eval, kind))
                    }
                    Token::Ampersand => None,
                    Token::At => None,

                    _ => unreachable!("parser should have dealt with an unknown unary operator way before here i think"),
                }
            }
            Expression::TypeCast { .. } => None,
            Expression::Group(sub) => Self::comptime_eval(sub),

            Expression::ParseError => None,
            Expression::FunctionCall { .. } => None,
            Expression::ArrayAccess { .. } => None,
            Expression::MemberAccess { .. } => None,
            Expression::StructInitializer { .. } => None,

            Expression::Identifier(_) => None,

            expr @ (Expression::String(_) | Expression::Integer(_, _) | Expression::Char(_)) => Some(expr.to_owned()),
        }
    }
    fn comptime_true(expr: &Expression) -> bool {
        match expr {
            expr @ (Expression::Binary { .. } | Expression::Unary { .. }) => match Self::comptime_eval(expr) {
                Some(Expression::Integer(x, _)) if x != 0 => true,
                _ => false,
            },
            Expression::TypeCast { .. } => false,
            Expression::Group(sub) => Self::comptime_true(sub),

            Expression::ParseError => false,
            Expression::FunctionCall { .. } => false,
            Expression::ArrayAccess { .. } => false,
            Expression::MemberAccess { .. } => false,
            Expression::StructInitializer { .. } => false,

            Expression::String(_) => false,
            Expression::Identifier(_) => false,

            Expression::Integer(x, _) => x != &0,
            Expression::Char(c) => c != &'\0',
        }
    }
    fn uniform_branch_return(block: &Statement) -> bool {
        match block {
            Statement::Return(_) => true,

            Statement::ParseError
                | Statement::Import(_)
                | Statement::VarDecl { .. }
                | Statement::Break
                | Statement::Continue
                | Statement::Asm(_)
                | Statement::Expression(_) => false,

            Statement::Block(stmts) => stmts.iter().any(|stmt| Self::uniform_branch_return(stmt)),
            Statement::If { condition, body, alt } => {
                Self::uniform_branch_return(&body) && match alt {
                    None => Analyzer::comptime_true(&condition),
                    Some(other) => Self::uniform_branch_return(other),
                }
            }
            Statement::While { condition, body } => Self::comptime_true(&condition) && Self::uniform_branch_return(&body),
        }
    }

    fn kindof(&mut self, expr: &Expression) -> Option<Type> {
        match expr {
            Expression::ParseError => None,

            Expression::Unary { child, .. } => self.kindof(child),
            Expression::Binary { lhs, .. } => self.kindof(lhs), // lhs must match rhs, so we can infer this i think
            Expression::FunctionCall { name, .. } => {
                match *name.to_owned() {
                    Expression::Identifier(ref n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find function named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind } = var else {
                            self.error(&format!("`{n}` is not callable"));
                            return None;
                        };
                        
                        Some(kind.to_owned())
                    }
                    _ => todo!(),
                }
            }
            Expression::ArrayAccess { lhs, .. } => {
                match *lhs.to_owned() {
                    Expression::Identifier(ref n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind: Type::Array { kind, .. } } = var else {
                            self.error(&format!("`{n}` is not an array"));
                            return None;
                        };

                        Some(*kind.to_owned())
                    }
                    _ => todo!(),
                }
            }
            Expression::MemberAccess { lhs, member } => {
                match *lhs.to_owned() {
                    Expression::Identifier(ref n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind } = var else {
                            self.error(&format!("`{n}` is not a variable"));
                            return None;
                        };
                        let struct_name = match kind {
                            Type::Ident(name) => name.clone(),
                            _ => {
                                self.error(&format!("`{n}` is not a struct"));
                                return None;
                            }
                        };
                        let Some(Symbol::Struct { fields }) = self.get_entry(&struct_name) else {
                            self.error(&format!("could not find struct type `{struct_name}`"));
                            return None;
                        };

                        fields.get(member).map(|x| x.to_owned())
                    }
                    _ => todo!(),
                }
            }
            Expression::StructInitializer { name, .. } => Some(Type::Ident(name.to_owned())),
            Expression::TypeCast { to, .. } => Some(*to.to_owned()),
            Expression::Integer(_, k) => Some(Type::Prim(k.to_owned())),
            Expression::String(_) => Some(Type::Pointer { kind: Box::from(Type::Prim("char".into())) }),
            Expression::Char(_) => Some(Type::Prim("char".into())),
            Expression::Identifier(x) => {
                let Some(Symbol::Variable { kind }) = self.get_entry(x) else {
                    self.error(&format!("cannot find variable named `{x}`"));
                    return None;
                };

                Some(kind.to_owned())
            }
            Expression::Group(child) => self.kindof(child),
        }
    }

    fn analyze_declaration(&mut self, decl: &Declaration) {
        match &decl {
            Declaration::ParseError => unreachable!(),
            Declaration::Enum { name, modifiers, variants } => {
                self.add_entry(name.into(), Symbol::Enum { mods: modifiers.clone(), variants: variants.clone() });
            }
            Declaration::Struct { name, fields } => {
                self.add_entry(name.into(), Symbol::Struct { fields: fields.clone() });
            }
            Declaration::Function { name, ret, params, body } => {
                self.add_entry(name.clone(),
                    Symbol::Variable {
                        kind: Type::Function {
                            ret: Box::new(ret.clone()),
                            params: params.iter().map(|x| x.1.clone()).collect() }
                    });

                self.add_table();
                for param in params {
                    self.add_entry(param.0.clone(), Symbol::Variable { kind: param.1.clone() });
                }

                if ret != &Type::Prim(String::from("void")) && !Self::uniform_branch_return(body) {
                    self.warning(&format!("{name}: not all branches return"));
                }

                self.analyze_statement(body, (false, true));
                self.pop_table();
            }
            Declaration::TypeDef { name, kind } => self.add_entry(name.into(), Symbol::Type { kind: kind.to_owned() }),
            Declaration::Statement(stmt) => self.analyze_statement(stmt, (false, false)),
        }
    }

                                                      // (breakable, returnable)
    fn analyze_statement(&mut self, body: &Statement, control_flow: (bool, bool)) {
        match body {
            Statement::ParseError => unreachable!(),
            Statement::VarDecl { name, kind, value, .. } => {
                if let None = kind && let None = value {
                    self.error("variable must have a type if no value is assigned");
                }
                let kind_final = match kind {
                    Some(k) => k.to_owned(),
                    None => {
                        let Some(k) = self.kindof(value.as_ref().unwrap()) else {
                            self.error(&format!("could not infer the type of `{name}`")); return;
                        };
                        k
                    }
                };
                self.add_entry(name.clone(), Symbol::Variable { kind: kind_final });
            }
            Statement::Import(path) => {
                if let Err(e) = std::fs::exists(path) {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => self.error(&format!("could not find file `{path}`")),
                        _ => self.error(&format!("file read error: `{e}`"))
                    }
                };
            }
            Statement::Return(expr) => if expr.is_some() { self.analyze_expression(expr.as_ref().unwrap()) }
            Statement::If { condition, body, alt } => {
                if !["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
                    .map(|x| Some(Type::Prim(x.into())))
                    .contains(&self.kindof(condition)) {
                        self.error("if condition must have integer type");
                }
                self.analyze_statement(body, control_flow);
                if let Some(alt_body) = alt {
                    self.analyze_statement(alt_body, control_flow);
                }
            }
            Statement::While { condition, body } => {
                if !["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
                    .map(|x| Some(Type::Prim(x.into())))
                    .contains(&self.kindof(condition)) {
                        self.error("if condition must have integer type");
                }
                self.analyze_statement(body, (true, control_flow.1));
            }
            Statement::Break => {
                if control_flow.0 {

                }
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::ParseError => unreachable!(),
            Expression::Unary { child, .. } => self.analyze_expression(child),
            _ => {}
        }
    }

    pub fn run(&mut self, root: &RootAST) {
        for decl in root {
            self.analyze_declaration(decl);
        }

        eprintln!("{:#?}", self.env);
    }
}
