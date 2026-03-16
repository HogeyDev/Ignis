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
}

type Environment = Vec<HashMap<String, Symbol>>;
pub struct Analyzer {
    env: Environment,
    pub err_count: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            err_count: 0,
        }
    }

    fn warning(&self, msg: &str) {
        eprintln!("\x1b[0;33mwarning\x1b[0;0m: {msg}");
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
    fn get_entry(&mut self, name: String) -> Option<&Symbol> {
        self.env.iter().rev().find_map(|table| table.get(&name))
    }

    fn comptime_eval(expr: &Expression) -> Option<Expression> {
        match expr {
            Expression::Binary { lhs, rhs, op } => {
                let eval_logic = |lhs: &Expression, rhs: &Expression, op: fn(bool, bool) -> bool| -> Option<Expression> {
                    let lhs_eval = Self::comptime_eval(lhs)? != Expression::Integer(0);
                    let rhs_eval = Self::comptime_eval(rhs)? != Expression::Integer(0);

                    Some(Expression::Integer(op(lhs_eval, rhs_eval) as i128))
                };
                let eval_math = |lhs: &Expression, rhs: &Expression, op: fn(i128, i128) -> i128| -> Option<Expression> {
                    let Expression::Integer(lhs_eval) = Self::comptime_eval(lhs)? else { unreachable!(); };
                    let Expression::Integer(rhs_eval) = Self::comptime_eval(rhs)? else { unreachable!(); };

                    Some(Expression::Integer(op(lhs_eval, rhs_eval)))
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
                        let eval = Self::comptime_eval(child)? != Expression::Integer(0);
                        Some(Expression::Integer((!eval) as i128))
                    }
                    Token::Minus => {
                        let Expression::Integer(eval) = Self::comptime_eval(child)? else { unreachable!(); };
                        Some(Expression::Integer(-eval))
                    }
                    Token::BitNeg => {
                        let Expression::Integer(eval) = Self::comptime_eval(child)? else { unreachable!(); };
                        Some(Expression::Integer(!eval))
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

            s @ Expression::String(_) => Some(s.to_owned()),
            x @ Expression::Integer(_) => Some(x.to_owned()),
            c @ Expression::Char(_) => Some(c.to_owned()),
        }
    }
    fn comptime_true(expr: &Expression) -> bool {
        match expr {
            expr @ (Expression::Binary { .. } | Expression::Unary { .. }) => match Self::comptime_eval(expr) {
                Some(Expression::Integer(x)) if x != 0 => true,
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

            Expression::Integer(x) => x != &0,
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
    fn analyze_declaration(&mut self, decl: &Declaration) {
        match &decl {
            &Declaration::ParseError => unreachable!(),
            &Declaration::Enum { name, modifiers, variants } => {
                self.add_entry(name.into(), Symbol::Enum { mods: modifiers.clone(), variants: variants.clone() });
            }
            &Declaration::Struct { name, fields } => {
                self.add_entry(name.into(), Symbol::Struct { fields: fields.clone() });
            }
            &Declaration::Function { name, ret, params, body } => {
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

                self.analyze_statement(body);
                self.pop_table();
            }
            &Declaration::TypeDef { name, kind } => {

            }
            &Declaration::Statement(stmt) => {}
        }
    }

    fn analyze_statement(&mut self, body: &Statement) {}

    pub fn run(&mut self, root: &RootAST) {
        for decl in root {
            self.analyze_declaration(decl);
        }

        // eprintln!("{:#?}", self.env);
    }
}
