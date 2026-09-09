use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use crate::{lexer::Token, parser::{Declaration, Expression, Parser, Statement, Type}};

#[derive(Debug)]
pub enum Symbol {
    Variable {
        kind: Rc<RefCell<Type>>,
    },
    Enum {
        mods: HashSet<String>,
        variants: Vec<String>,
    },
    Struct {
        fields: HashMap<String, Rc<RefCell<Type>>>,
    },
    Type {
        kind: Rc<RefCell<Type>>,
    }
}

type Environment = Vec<HashMap<String, Rc<RefCell<Symbol>>>>;
pub struct Analyzer<'a> {
    pub parser: Parser<'a>,
    env: Environment,

    pub warn_count: usize,
    pub err_count: usize,
}

impl<'a> Analyzer<'a> {
    pub fn from(parser: Parser<'a>) -> Self {
        Self {
            env: Environment::new(),
            parser,

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
    fn add_entry(&mut self, name: String, symbol: Rc<RefCell<Symbol>>) {
        if self.env.is_empty() {
            self.add_table();
        }
        if self.env.last_mut().unwrap().insert(name.clone(), symbol).is_some() {
            eprintln!("\x1b[0;31merror\x1b[0;0m: redefinition of {}", name);
        }
    }
    fn get_entry(&self, name: &String) -> Option<Rc<RefCell<Symbol>>> {
        self.env.iter().rev().find_map(|table| table.get(name)).map(|x| x.clone())
    }

    fn comptime_eval(&self, expr: Rc<RefCell<Expression>>) -> Option<Rc<RefCell<Expression>>> {
        match &*expr.borrow() {
            Expression::Binary { lhs, rhs, op } => {
                let eval_logic = |lhs: Rc<RefCell<Expression>>, rhs: Rc<RefCell<Expression>>, op: fn(bool, bool) -> bool| -> Option<Rc<RefCell<Expression>>> {
                    let lhs_eval = {
                        let Expression::Integer(val, _) = *self.comptime_eval(lhs)?.borrow() else { unreachable!(); };
                        val != 0
                    };
                    let rhs_eval = {
                        let Expression::Integer(val, _) = *self.comptime_eval(rhs)?.borrow() else { unreachable!(); };
                        val != 0
                    };

                    Some(Rc::new(RefCell::new(Expression::Integer(op(lhs_eval, rhs_eval) as i128, "i32".into()))))
                };
                let eval_math = |lhs: Rc<RefCell<Expression>>, rhs: Rc<RefCell<Expression>>, op: fn(i128, i128) -> i128| -> Option<Rc<RefCell<Expression>>> {
                    let Expression::Integer(lhs_eval, _) = *self.comptime_eval(lhs)?.borrow() else { unreachable!(); };
                    let Expression::Integer(rhs_eval, _) = *self.comptime_eval(rhs)?.borrow() else { unreachable!(); };

                    Some(Rc::new(RefCell::new(Expression::Integer(op(lhs_eval, rhs_eval), "i32".into()))))
                };
                match op {
                    Token::Equals => self.comptime_eval(rhs.clone()),
                    Token::LogOr => eval_logic(lhs.clone(), rhs.clone(), |l, r| l || r),
                    Token::LogAnd => eval_logic(lhs.clone(), rhs.clone(), |l, r| l && r),
                    Token::DoubleEquals => eval_logic(lhs.clone(), rhs.clone(), |l, r| l == r),
                    Token::NotEquals => eval_logic(lhs.clone(), rhs.clone(), |l, r| l != r),
                    Token::LessThan => eval_logic(lhs.clone(), rhs.clone(), |l, r| l < r),
                    Token::MoreThan => eval_logic(lhs.clone(), rhs.clone(), |l, r| l > r),
                    Token::LessThanEq => eval_logic(lhs.clone(), rhs.clone(), |l, r| l <= r),
                    Token::MoreThanEq => eval_logic(lhs.clone(), rhs.clone(), |l, r| l >= r),

                    Token::BitOr => eval_math(lhs.clone(), rhs.clone(), |l, r| l | r),
                    Token::BitXor => eval_math(lhs.clone(), rhs.clone(), |l, r| l ^ r),
                    Token::Ampersand => eval_math(lhs.clone(), rhs.clone(), |l, r| l & r),
                    Token::LShift => eval_math(lhs.clone(), rhs.clone(), |l, r| l << r),
                    Token::RShift => eval_math(lhs.clone(), rhs.clone(), |l, r| l >> r),
                    Token::Plus => eval_math(lhs.clone(), rhs.clone(), |l, r| l + r),
                    Token::Minus => eval_math(lhs.clone(), rhs.clone(), |l, r| l - r),
                    Token::Star => eval_math(lhs.clone(), rhs.clone(), |l, r| l * r),
                    Token::Slash => eval_math(lhs.clone(), rhs.clone(), |l, r| l / r),
                    Token::Percent => eval_math(lhs.clone(), rhs.clone(), |l, r| l % r),

                    _ => unreachable!("parser should have dealt with an unknown binary operator way before here i think"),
                }
            }
            Expression::Unary { child, op } => {
                match op {
                    Token::LogNot => {
                        let (eval, kind) = {
                            let borrowed = self.comptime_eval(child.clone())?;
                            let Expression::Integer(value, ref kind) = *borrowed.borrow() else { unreachable!(); };
                            (value != 0, kind.clone())
                        };
                        Some(Rc::new(RefCell::new(Expression::Integer((!eval) as i128, kind))))
                    }
                    Token::Minus => {
                        let borrowed = self.comptime_eval(child.clone())?;
                        let Expression::Integer(eval, ref kind) = *borrowed.borrow() else { unreachable!(); };
                        Some(Rc::new(RefCell::new(Expression::Integer((-eval) as i128, kind.clone()))))
                    }
                    Token::BitNeg => {
                        let borrowed = self.comptime_eval(child.clone())?;
                        let Expression::Integer(eval, ref kind) = *borrowed.borrow() else { unreachable!(); };
                        Some(Rc::new(RefCell::new(Expression::Integer(!eval, kind.clone()))))
                    }
                    Token::Ampersand => None,
                    Token::At => None,

                    _ => unreachable!("parser should have dealt with an unknown unary operator way before here i think"),
                }
            }
            Expression::TypeCast { .. } => None,
            Expression::Group(sub) => self.comptime_eval(sub.clone()),

            Expression::ParseError => None,
            Expression::FunctionCall { .. } => None,
            Expression::ArrayAccess { .. } => None,
            Expression::MemberAccess { .. } => None,
            Expression::StructInitializer { .. } => None,

            Expression::Identifier(_) => None,

            expr @ (Expression::String(_) | Expression::Integer(_, _) | Expression::Char(_)) => Some(Rc::new(RefCell::new(expr.to_owned()))),
        }
    }
    fn comptime_true(&self, expr: Rc<RefCell<Expression>>) -> bool {
        match &*expr.borrow() {
            Expression::Binary { .. } | Expression::Unary { .. } => self.comptime_eval(expr.clone())
                .is_some_and(|eval| matches!(&*eval.borrow(), Expression::Integer(x, _) if *x != 0)),
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
    fn uniform_branch_return(&self, block: Rc<RefCell<Statement>>) -> bool {
        match &*block.borrow() {
            Statement::Return(_) => true,

            Statement::ParseError
                | Statement::Import(_)
                | Statement::VarDecl { .. }
                | Statement::Break
                | Statement::Continue
                | Statement::Asm(_)
                | Statement::Expression(_) => false,

            Statement::Block(stmts) => stmts.iter().any(|stmt| self.uniform_branch_return(stmt.clone())),
            Statement::If { condition, body, alt } => {
                self.uniform_branch_return(body.clone()) && match alt {
                    None => self.comptime_true(condition.clone()),
                    Some(other) => self.uniform_branch_return(other.clone()),
                }
            }
            Statement::While { condition, body } => self.comptime_true(condition.clone()) && self.uniform_branch_return(body.clone()),
        }
    }

    fn kindof(&mut self, expr: Rc<RefCell<Expression>>) -> Option<Rc<RefCell<Type>>> {
        match &*expr.borrow() {
            Expression::ParseError => None,

            Expression::Unary { child, .. } => self.kindof(child.clone()),
            Expression::Binary { lhs, .. } => self.kindof(lhs.clone()), // lhs must match rhs, so we can infer this i think
            Expression::FunctionCall { name, .. } => {
                match &*name.borrow() {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find function named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind } = &*var.borrow() else {
                            self.error(&format!("`{n}` is not callable"));
                            return None;
                        };
                        
                        Some(kind.clone())
                    }
                    _ => todo!(),
                }
            }
            Expression::ArrayAccess { lhs, .. } => {
                match &*lhs.borrow() {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind: var_kind } = &*var.borrow() else {
                            self.error(&format!("`{n}` is not a variable"));
                            return None;
                        };
                        let Type::Array { kind, .. } = &*var_kind.borrow() else {
                            self.error(&format!("`{n} is not an array"));
                            return None;
                        };

                        Some(kind.clone())
                    }
                    _ => todo!(),
                }
            }
            Expression::MemberAccess { lhs, member } => {
                match &*lhs.borrow() {
                    Expression::Identifier(n) => {
                        let Some(var) = self.get_entry(n) else {
                            self.error(&format!("cannot find variable named `{n}`"));
                            return None;
                        };
                        let Symbol::Variable { kind: var_kind } = &*var.borrow() else {
                            self.error(&format!("`{n}` is not a variable"));
                            return None;
                        };
                        let struct_name = match &*var_kind.borrow() {
                            Type::Ident(name) => name.clone(),
                            _ => {
                                self.error(&format!("`{n}` is not a struct"));
                                return None;
                            }
                        };
                        let entry = self.get_entry(&struct_name);
                        if let Some(symbol) = entry && let Symbol::Struct { fields } = &*symbol.borrow() {
                            fields.get(member).map(|x| x.to_owned())
                        } else {
                            self.error(&format!("could not find struct type `{struct_name}`"));
                            None
                        }
                    }
                    _ => todo!(),
                }
            }
            Expression::StructInitializer { name, .. } => Some(Rc::new(RefCell::new(Type::Ident(name.to_owned())))),
            Expression::TypeCast { to, .. } => Some(to.clone()),
            Expression::Integer(_, k) => Some(Rc::new(RefCell::new(Type::Prim(k.to_owned())))),
            Expression::String(_) => {
                let char_id = Rc::new(RefCell::new(Type::Prim("char".into())));
                Some(Rc::new(RefCell::new(Type::Pointer { kind: char_id })))
            }
            Expression::Char(_) => Some(Rc::new(RefCell::new(Type::Prim("char".into())))),
            Expression::Identifier(x) => {
                let entry = self.get_entry(x);
                if let Some(symbol) = entry && let Symbol::Variable { kind } = &*symbol.borrow() {
                    Some(kind.to_owned())
                } else {
                    self.error(&format!("cannot find variable named `{x}`"));
                    None
                }
            }
            Expression::Group(child) => self.kindof(child.clone()),
        }
    }

    fn analyze_declaration(&mut self, decl: Rc<RefCell<Declaration>>) {
        match &*decl.borrow() {
            Declaration::ParseError => unreachable!(),
            Declaration::Enum { name, modifiers, variants } => {
                self.add_entry(name.clone(), Rc::new(RefCell::new(Symbol::Enum { mods: modifiers.clone(), variants: variants.clone() })));
            }
            Declaration::Struct { name, fields } => {
                self.add_entry(name.clone(), Rc::new(RefCell::new(Symbol::Struct { fields: fields.clone() })));
            }
            Declaration::Function { name, ret, params, body } => {
                let kind = Rc::new(RefCell::new(Type::Function {
                    ret: ret.clone(),
                    params: params.iter().map(|x| x.1.clone()).collect(),
                }));
                self.add_entry(name.clone(),
                    Rc::new(RefCell::new(Symbol::Variable { kind })));

                self.add_table();
                for param in params {
                    let name = param.0.clone();
                    self.add_entry(name, Rc::new(RefCell::new(Symbol::Variable { kind: param.1.clone() })));
                }

                if *ret.borrow() != Type::Prim(String::from("void")) && !self.uniform_branch_return(body.clone()) {
                    self.warning(&format!("{name}: not all branches return"));
                }

                self.analyze_statement(body.clone(), (false, true));
                self.pop_table();
            }
            Declaration::TypeDef { name, kind } => self.add_entry(name.clone(), Rc::new(RefCell::new(Symbol::Type { kind: kind.clone() }))),
            Declaration::Statement(stmt) => self.analyze_statement(stmt.clone(), (false, false)),
        }
    }

                                                       // (breakable, returnable)
    fn analyze_statement(&mut self, body: Rc<RefCell<Statement>>, control_flow: (bool, bool)) {
        match &*body.borrow() {
            Statement::ParseError => unreachable!(),
            Statement::VarDecl { name, kind, value, .. } => {
                if let None = kind && let None = value {
                    self.error("variable must have a type if no value is assigned");
                }
                let kind_final = match kind {
                    Some(k) => k.to_owned(),
                    None => {
                        let Some(k) = self.kindof(value.as_ref().unwrap().clone()) else {
                            self.error(&format!("could not infer the type of `{name}`")); return;
                        };
                        k
                    }
                };
                self.add_entry(name.clone(), Rc::new(RefCell::new(Symbol::Variable { kind: kind_final })));
            }
            Statement::Import(path) => {
                if let Err(e) = std::fs::exists(path) {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => self.error(&format!("could not find file `{path}`")),
                        _ => self.error(&format!("file read error: `{e}`"))
                    }
                };
            }
            Statement::Return(expr) => {
                if expr.is_some() {
                    _ = self.analyze_expression(expr.as_ref().unwrap().clone())
                }
            }
            Statement::If { condition, body, alt } => {
                if let Some(kind) = self.kindof(condition.clone()) && kind.borrow().is_integer() {
                    self.analyze_statement(body.clone(), control_flow);
                    if let Some(alt_body) = alt {
                        self.analyze_statement(alt_body.clone(), control_flow);
                    }
                } else {
                    self.error("if condition must have integer type");
                }
            }
            Statement::While { condition, body } => {
                if let Some(kind) = self.kindof(condition.clone()) && kind.borrow().is_integer() {
                    self.analyze_statement(body.clone(), (true, control_flow.1));
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
            Statement::Block(stmts) => stmts.iter().for_each(|stmt| self.analyze_statement(stmt.clone(), control_flow)),
            Statement::Expression(expr) => _ = self.analyze_expression(expr.clone()),
        }
    }

    fn analyze_expression(&mut self, expr: Rc<RefCell<Expression>>) -> Option<Rc<RefCell<Type>>> {
        match &*expr.borrow() {
            Expression::ParseError => unreachable!(),
            Expression::Unary { child, op } => {
                self.analyze_expression(child.clone());
                match op {
                    Token::At => match self.kindof(child.clone()) {
                        Some(ptr) if matches!(&*ptr.borrow(), Type::Pointer { .. }) => Some(ptr),
                        _ => {
                            self.error("cannot deference non pointer type");
                            None
                        }
                    }
                    Token::Ampersand => match &*child.borrow() {
                        Expression::Identifier(_) | Expression::MemberAccess { .. } | Expression::ArrayAccess { .. } => {
                            let ptr_kind = self.kindof(child.clone())?;
                            Some(Rc::new(RefCell::new(Type::Pointer { kind: ptr_kind })))
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
                let lhs_type = self.analyze_expression(lhs.clone());
                let rhs_type = self.analyze_expression(rhs.clone());

                if matches!(lhs_type, Some(ref ptr) if matches!(&*ptr.borrow(), Type::Pointer { .. })) &&
                    let Some(rhs_type_unwrap) = rhs_type && rhs_type_unwrap.borrow().is_integer() { lhs_type }
                else if lhs_type.is_some() { lhs_type }
                else { None }
            }
            Expression::FunctionCall { .. } => self.kindof(expr.clone()),
            Expression::ArrayAccess { lhs, index } => {
                let lhs_type = self.kindof(lhs.clone());
                match lhs_type {
                    Some(arr_type) => {
                        if let Type::Array { size, kind } = &*arr_type.borrow() {
                            if let Some(size_expr) = size &&
                                let Some(Expression::Integer(size_val, _)) = self.comptime_eval(size_expr.clone()).map(|x| x.borrow().clone()) &&
                                let Some(Expression::Integer(index_val, _)) = self.comptime_eval(index.clone()).map(|x| x.borrow().clone()) &&
                                (index_val < 0 || index_val >= size_val)
                            {
                                self.error("array index out of bounds");
                            }
                            if let Some(index_type) = self.kindof(index.clone()) && index_type.borrow().is_integer() {
                                Some(kind.clone())
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
            Expression::MemberAccess { .. } => self.kindof(expr.clone()),
            Expression::StructInitializer { name, .. } => Some(Rc::new(RefCell::new(Type::Ident(name.clone())))),
            Expression::TypeCast { to, .. } => Some(to.clone()),
            Expression::Integer(_, kind) => Some(Rc::new(RefCell::new(Type::Prim(kind.to_owned())))),
            Expression::String(_) => {
                let char_id = Rc::new(RefCell::new(Type::Prim("char".to_owned())));
                Some(Rc::new(RefCell::new(Type::Array { size: None, kind: char_id })))
            }
            Expression::Char(_) => Some(Rc::new(RefCell::new(Type::Prim("char".to_owned())))),
            Expression::Identifier(_) => self.kindof(expr.clone()),
            Expression::Group(child) => self.kindof(child.clone()),
        }
    }

    pub fn run(&mut self, root: &[Rc<RefCell<Declaration>>]) {
        for decl in root {
            self.analyze_declaration(decl.clone());
        }

        // eprintln!("{:#?}", self.env);
    }
}
