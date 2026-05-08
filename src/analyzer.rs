use std::collections::{HashMap, HashSet};

use crate::{lexer::Token, parser::{Declaration, DeclarationId, Expression, ExpressionId, Parser, RootAst, Statement, StatementId, Type, TypeId}};

type SymbolId = usize;
#[derive(Debug)]
pub enum Symbol {
    Variable {
        kind: TypeId,
    },
    Enum {
        mods: HashSet<String>,
        variants: Vec<String>,
    },
    Struct {
        fields: HashMap<String, TypeId>,
    },
    Type {
        kind: TypeId,
    }
}

type Environment = Vec<HashMap<String, SymbolId>>;
pub struct Analyzer<'a> {
    parser: Parser<'a>,
    env: Environment,
    symbol_arena: Vec<Symbol>,

    pub warn_count: usize,
    pub err_count: usize,
}

impl<'a> Analyzer<'a> {
    pub fn from(parser: Parser<'a>) -> Self {
        Self {
            env: Environment::new(),
            parser,
            symbol_arena: Vec::new(),

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
        let id = self.symbol_arena.len();
        self.symbol_arena.push(symbol);

        if self.env.last_mut().unwrap().insert(name.clone(), id).is_some() {
            eprintln!("\x1b[0;31merror\x1b[0;0m: redefinition of {}", name);
        }
    }
    fn get_entry(&self, name: &String) -> Option<SymbolId> {
        self.env.iter().rev().find_map(|table| table.get(name)).map(|x| *x)
    }

    fn comptime_eval(&self, expr_id: ExpressionId) -> Option<Expression> {
        let expr = &self.parser.expression_arena[expr_id];
        match expr {
            Expression::Binary { lhs: lhs_id, rhs: rhs_id, op } => {
                let eval_logic = |lhs: ExpressionId, rhs: ExpressionId, op: fn(bool, bool) -> bool| -> Option<Expression> {
                    let lhs_eval = {
                        let Expression::Integer(val, _) = self.comptime_eval(lhs)? else { unreachable!(); };
                        val != 0
                    };
                    let rhs_eval = {
                        let Expression::Integer(val, _) = self.comptime_eval(rhs)? else { unreachable!(); };
                        val != 0
                    };

                    Some(Expression::Integer(op(lhs_eval, rhs_eval) as i128, "i32".into()))
                };
                let eval_math = |lhs: ExpressionId, rhs: ExpressionId, op: fn(i128, i128) -> i128| -> Option<Expression> {
                    let Expression::Integer(lhs_eval, _) = self.comptime_eval(lhs)? else { unreachable!(); };
                    let Expression::Integer(rhs_eval, _) = self.comptime_eval(rhs)? else { unreachable!(); };

                    Some(Expression::Integer(op(lhs_eval, rhs_eval), "i32".into()))
                };
                match op {
                    Token::Equals => self.comptime_eval(*rhs_id),
                    Token::LogOr => eval_logic(*lhs_id, *rhs_id, |l, r| l || r),
                    Token::LogAnd => eval_logic(*lhs_id, *rhs_id, |l, r| l && r),
                    Token::DoubleEquals => eval_logic(*lhs_id, *rhs_id, |l, r| l == r),
                    Token::NotEquals => eval_logic(*lhs_id, *rhs_id, |l, r| l != r),
                    Token::LessThan => eval_logic(*lhs_id, *rhs_id, |l, r| l < r),
                    Token::MoreThan => eval_logic(*lhs_id, *rhs_id, |l, r| l > r),
                    Token::LessThanEq => eval_logic(*lhs_id, *rhs_id, |l, r| l <= r),
                    Token::MoreThanEq => eval_logic(*lhs_id, *rhs_id, |l, r| l >= r),

                    Token::BitOr => eval_math(*lhs_id, *rhs_id, |l, r| l | r),
                    Token::BitXor => eval_math(*lhs_id, *rhs_id, |l, r| l ^ r),
                    Token::Ampersand => eval_math(*lhs_id, *rhs_id, |l, r| l & r),
                    Token::LShift => eval_math(*lhs_id, *rhs_id, |l, r| l << r),
                    Token::RShift => eval_math(*lhs_id, *rhs_id, |l, r| l >> r),
                    Token::Plus => eval_math(*lhs_id, *rhs_id, |l, r| l + r),
                    Token::Minus => eval_math(*lhs_id, *rhs_id, |l, r| l - r),
                    Token::Star => eval_math(*lhs_id, *rhs_id, |l, r| l * r),
                    Token::Slash => eval_math(*lhs_id, *rhs_id, |l, r| l / r),
                    Token::Percent => eval_math(*lhs_id, *rhs_id, |l, r| l % r),

                    _ => unreachable!("parser should have dealt with an unknown binary operator way before here i think"),
                }
            }
            Expression::Unary { child: child_id, op } => {
                match op {
                    Token::LogNot => {
                        let (eval, kind) = {
                            let Expression::Integer(v, k) = self.comptime_eval(*child_id)? else { unreachable!(); };
                            (v != 0, k)
                        };
                        Some(Expression::Integer((!eval) as i128, kind))
                    }
                    Token::Minus => {
                        let Expression::Integer(eval, kind) = self.comptime_eval(*child_id)? else { unreachable!(); };
                        Some(Expression::Integer((-eval) as i128, kind))
                    }
                    Token::BitNeg => {
                        let Expression::Integer(eval, kind) = self.comptime_eval(*child_id)? else { unreachable!(); };
                        Some(Expression::Integer(!eval, kind))
                    }
                    Token::Ampersand => None,
                    Token::At => None,

                    _ => unreachable!("parser should have dealt with an unknown unary operator way before here i think"),
                }
            }
            Expression::TypeCast { .. } => None,
            Expression::Group(sub) => self.comptime_eval(*sub),

            Expression::ParseError => None,
            Expression::FunctionCall { .. } => None,
            Expression::ArrayAccess { .. } => None,
            Expression::MemberAccess { .. } => None,
            Expression::StructInitializer { .. } => None,

            Expression::Identifier(_) => None,

            expr @ (Expression::String(_) | Expression::Integer(_, _) | Expression::Char(_)) => Some(expr.to_owned()),
        }
    }
    fn comptime_true(&self, expr_id: ExpressionId) -> bool {
        let expr = &self.parser.expression_arena[expr_id];
        match expr {
            Expression::Binary { .. } | Expression::Unary { .. } => match self.comptime_eval(expr_id) {
                Some(Expression::Integer(x, _)) if x != 0 => true,
                _ => false,
            },
            Expression::TypeCast { .. } => false,
            Expression::Group(sub) => self.comptime_true(sub.clone()),

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
    fn uniform_branch_return(&self, block_id: StatementId) -> bool {
        let block = &self.parser.statement_arena[block_id];
        match block {
            Statement::Return(_) => true,

            Statement::ParseError
                | Statement::Import(_)
                | Statement::VarDecl { .. }
                | Statement::Break
                | Statement::Continue
                | Statement::Asm(_)
                | Statement::Expression(_) => false,

            Statement::Block(stmts) => stmts.iter().any(|stmt| self.uniform_branch_return(*stmt)),
            Statement::If { condition, body, alt } => {
                self.uniform_branch_return(*body) && match alt {
                    None => self.comptime_true(*condition),
                    Some(other) => self.uniform_branch_return(*other),
                }
            }
            Statement::While { condition, body } => self.comptime_true(*condition) && self.uniform_branch_return(*body),
        }
    }

    fn kindof(&mut self, expr_id: ExpressionId) -> Option<TypeId> {
        let expr = &self.parser.expression_arena[expr_id];
        match expr {
            Expression::ParseError => None,

            Expression::Unary { child, .. } => self.kindof(*child),
            Expression::Binary { lhs, .. } => self.kindof(*lhs), // lhs must match rhs, so we can infer this i think
            Expression::FunctionCall { name: name_id, .. } => {
                match &self.parser.expression_arena[*name_id] {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find function named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind } = &self.symbol_arena[var] else {
                            self.error(&format!("`{n}` is not callable"));
                            return None;
                        };
                        
                        Some(*kind)
                    }
                    _ => todo!(),
                }
            }
            Expression::ArrayAccess { lhs: lhs_id, .. } => {
                let lhs = &self.parser.expression_arena[*lhs_id];
                match lhs {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind: kind_id } = &self.symbol_arena[var] else {
                            self.error(&format!("`{n}` is not a variable"));
                            return None;
                        };
                        let Type::Array { kind, .. } = self.parser.type_arena[*kind_id] else {
                            self.error(&format!("`{n} is not an array"));
                            return None;
                        };

                        Some(kind)
                    }
                    _ => todo!(),
                }
            }
            Expression::MemberAccess { lhs: lhs_id, member } => {
                let lhs = &self.parser.expression_arena[*lhs_id];
                match lhs {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind: kind_id } = &self.symbol_arena[var] else {
                            self.error(&format!("`{n}` is not a variable"));
                            return None;
                        };
                        let kind = &self.parser.type_arena[*kind_id];
                        let struct_name = match kind {
                            Type::Ident(name) => name.clone(),
                            _ => {
                                self.error(&format!("`{n}` is not a struct"));
                                return None;
                            }
                        };
                        let entry = self.get_entry(&struct_name);
                        if let Some(entry_id) = entry && let Symbol::Struct { fields } = &self.symbol_arena[entry_id] {
                            fields.get(member).map(|x| x.to_owned())
                        } else {
                            self.error(&format!("could not find struct type `{struct_name}`"));
                            None
                        }
                    }
                    _ => todo!(),
                }
            }
            Expression::StructInitializer { name, .. } => Some(self.parser.add_type(Type::Ident(name.to_owned()))),
            Expression::TypeCast { to, .. } => Some(*to),
            Expression::Integer(_, k) => Some(self.parser.add_type(Type::Prim(k.to_owned()))),
            Expression::String(_) => {
                let char_id = self.parser.add_type(Type::Prim("char".into()));
                Some(self.parser.add_type(Type::Pointer { kind: char_id }))
            }
            Expression::Char(_) => Some(self.parser.add_type(Type::Prim("char".into()))),
            Expression::Identifier(x) => {
                let entry = self.get_entry(x);
                if let Some(entry_id) = entry && let Symbol::Variable { kind } = &self.symbol_arena[entry_id] {
                    Some(kind.to_owned())
                } else {
                    self.error(&format!("cannot find variable named `{x}`"));
                    None
                }
            }
            Expression::Group(child) => self.kindof(*child),
        }
    }

    fn analyze_declaration(&mut self, decl_id: DeclarationId) {
        let decl = self.parser.declaration_arena[decl_id].clone();
        match decl {
            Declaration::ParseError => unreachable!(),
            Declaration::Enum { name, modifiers, variants } => {
                self.add_entry(name.into(), Symbol::Enum { mods: modifiers.clone(), variants: variants.clone() });
            }
            Declaration::Struct { name, fields } => {
                self.add_entry(name.into(), Symbol::Struct { fields: fields.clone() });
            }
            Declaration::Function { name, ret, params, body } => {
                let params_tmp = params.iter().map(|x| x.1).collect();
                let kind = self.parser.add_type(Type::Function {
                    ret,
                    params: params_tmp,
                });
                self.add_entry(name.clone(),
                    Symbol::Variable { kind });

                self.add_table();
                for param in params {
                    let name = param.0.clone();
                    self.add_entry(name, Symbol::Variable { kind: param.1 });
                }

                let ret_kind = &self.parser.type_arena[ret];
                if ret_kind != &Type::Prim(String::from("void")) && !self.uniform_branch_return(body) {
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
    fn analyze_statement(&mut self, body_id: StatementId, control_flow: (bool, bool)) {
        let body = self.parser.statement_arena[body_id].clone();
        match body {
            Statement::ParseError => unreachable!(),
            Statement::VarDecl { name, kind, value, .. } => {
                if let None = kind && let None = value {
                    self.error("variable must have a type if no value is assigned");
                }
                let kind_final = match kind {
                    Some(k) => k.to_owned(),
                    None => {
                        let Some(k) = self.kindof(value.unwrap()) else {
                            self.error(&format!("could not infer the type of `{name}`")); return;
                        };
                        k
                    }
                };
                self.add_entry(name.clone(), Symbol::Variable { kind: kind_final });
            }
            Statement::Import(ref path) => {
                if let Err(e) = std::fs::exists(path) {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => self.error(&format!("could not find file `{path}`")),
                        _ => self.error(&format!("file read error: `{e}`"))
                    }
                };
            }
            Statement::Return(expr) => {
                if expr.is_some() {
                    _ = self.analyze_expression(expr.unwrap())
                }
            }
            Statement::If { condition, body, alt } => {
                if let Some(kind_id) = self.kindof(condition) && self.parser.type_arena[kind_id].is_integer() {
                    self.analyze_statement(body, control_flow);
                    if let Some(alt_body) = alt {
                        self.analyze_statement(alt_body, control_flow);
                    }
                } else {
                    self.error("if condition must have integer type");
                }
            }
            Statement::While { condition, body } => {
                if let Some(kind_id) = self.kindof(condition) && self.parser.type_arena[kind_id].is_integer() {
                    self.analyze_statement(body, (true, control_flow.1));
                } else {
                    self.error("while condition must have integer type");
                }
            }
            Statement::Break => {
                if !control_flow.0 {
                    self.error("break statement cannot be used here");
                }
            }
            Statement::Continue => {
                if !control_flow.0 {
                    self.error("continue statement cannot be used here");
                }
            }
            Statement::Asm(_) => {}
            Statement::Block(stmts) => stmts.iter().for_each(|stmt| self.analyze_statement(*stmt, control_flow)),
            Statement::Expression(expr) => _ = self.analyze_expression(expr),
        }
    }

    fn analyze_expression(&mut self, expr_id: ExpressionId) -> Option<TypeId> {
        let expr = self.parser.expression_arena[expr_id].clone();
        match expr {
            Expression::ParseError => unreachable!(),
            Expression::Unary { child, op } => {
                self.analyze_expression(child);
                match op {
                    Token::At => match self.kindof(child) {
                        ptr @ Some(ptr_id) if matches!(self.parser.type_arena[ptr_id], Type::Pointer { .. }) => ptr,
                        _ => {
                            self.error("cannot deference non pointer type");
                            None
                        }
                    }
                    Token::Ampersand => match &self.parser.expression_arena[child] {
                        Expression::Identifier(_) => {
                            let ptr_kind = self.kindof(child)?;
                            Some(self.parser.add_type(Type::Pointer { kind: ptr_kind }))
                        }
                        Expression::MemberAccess { .. } => {
                            let ptr_kind = self.kindof(child)?;
                            Some(self.parser.add_type(Type::Pointer { kind: ptr_kind }))
                        }
                        Expression::ArrayAccess { .. } => {
                            let ptr_kind = self.kindof(child)?;
                            Some(self.parser.add_type(Type::Pointer { kind: ptr_kind }))
                        }
                        _ => {
                            self.error("invalid operand for reference operator");
                            None
                        }
                    }
                    _ => None
                }
            }
            Expression::Binary { lhs, rhs, .. } => {
                let lhs_type = self.analyze_expression(lhs);
                let rhs_type = self.analyze_expression(rhs);

                if matches!(lhs_type, Some(ptr_id) if matches!(self.parser.type_arena[ptr_id], Type::Pointer { .. })) &&
                    let Some(rhs_type_unwrap) = rhs_type && self.parser.type_arena[rhs_type_unwrap].is_integer() { lhs_type }
                else if lhs_type.is_some() { lhs_type }
                else { None }
            }
            Expression::FunctionCall { .. } => self.kindof(expr_id),
            Expression::ArrayAccess { lhs, index } => {
                let lhs_type = self.kindof(lhs);
                match lhs_type {
                    Some(arr_id) => {
                        if let Type::Array { size: size_id, kind: kind_id } = self.parser.type_arena[arr_id] {
                            if let Some(size_id_value) = size_id &&
                                let Some(Expression::Integer(size_val, _)) = self.comptime_eval(size_id_value) &&
                                let Some(Expression::Integer(index_val, _)) = self.comptime_eval(index) &&
                                (index_val < 0 || index_val >= size_val)
                            {
                                self.error("array index out of bounds");
                            }
                            if let Some(index_unwrap) = self.kindof(index) && self.parser.type_arena[index_unwrap].is_integer() {
                                Some(kind_id)
                            } else {
                                self.error("array index must be integer type");
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            // TODO: everything from here on down only does typechecking, this needs to be expanded
            Expression::MemberAccess { .. } => self.kindof(expr_id),
            Expression::StructInitializer { name, .. } => Some(self.parser.add_type(Type::Ident(name.clone()))),
            Expression::TypeCast { to, .. } => Some(to),
            Expression::Integer(_, kind) => Some(self.parser.add_type(Type::Prim(kind.to_owned()))),
            Expression::String(_) => {
                let char_id = self.parser.add_type(Type::Prim("char".to_owned()));
                Some(self.parser.add_type(Type::Array { size: None, kind: char_id }))
            }
            Expression::Char(_) => Some(self.parser.add_type(Type::Prim("char".to_owned()))),
            Expression::Identifier(_) => self.kindof(expr_id),
            Expression::Group(child) => self.kindof(child),
        }
    }

    pub fn run(&mut self, root: &RootAst) {
        for decl in root {
            self.analyze_declaration(*decl);
        }

        // eprintln!("{:#?}", self.env);
    }
}
