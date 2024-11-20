use std::{process, usize};

use crate::{io::SourceFile, lexer::{Token, TokenType, Tokenizer}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    Inc, // ++
    Dec, // --

    Inv, // !
    Neg, // -

    Or,  // ||
    And, // &&

    Eq,  // ==
    Neq, // !=
    LT,  // <
    GT,  // >
    LTE, // <=
    GTE, // >=

    ArrAcc, // []
    Ref,    // &
    Deref,  // @

    Assign, // =
}

#[derive(Debug, Clone)]
pub enum AST {
    Null,
    Integer(i64),
    String(String),
    Character(char),
    UnaryExpression {
        op: Operation,
        child: Box<AST>,
    },
    BinaryExpression {
        op: Operation,
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Argument(Box<AST>),
    Parameter {
        param_type: String,
        name: String,
    },
    FunctionDeclaration {
        name: String,
        return_type: String,
        prototype: Vec<Box<AST>>,
        body: Box<AST>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Box<AST>>,
    },
    VariableDeclaration {
        variable_type: String,
        name: String,
        is_static: bool,
    },
    VariableAssignment {
        name: String,
        value: Box<AST>,
    },
    VariableCall {
        name: String,
    },
    If {
        condition: Box<AST>,
        body: Box<AST>,
        alt: Option<Box<AST>>,
    },
    While {
        condition: Box<AST>,
        body: Box<AST>,
    },
    For {
        initializer: Box<AST>,
        condition: Box<AST>,
        updater: Box<AST>,
        body: Box<AST>,
    },
    Return(Box<AST>),
    Asm(String),
    Block(Vec<Box<AST>>),
    Import {
        module: String,
    },
    Struct {
        name: String,
        members: Vec<(String, String)>, // [NAME, TYPE]
    },
    Enum {
        name: String,
        values: Vec<String>, // [NAME]
        attributes: Vec<String>, // [NAME]
    },
    StructInitializer {
        spreads: bool,
        name: String,
        members: Vec<(String, Box<AST>)>, // [NAME, VALUE]
    },
    MemberAccess {
        accessed: Box<AST>,
        member: String,
    },
    Definition {
        name: String,
        value: Box<AST>,
    },
    Macro { // this is very similar to a function call lmao
        name: String,
        parameters: Vec<String>,
        expansion: Vec<(bool, String)>,
    },
    TypeDefinition {
        name: String,
        type_string: String,
    },
}

impl AST {
    pub fn to_string(&self) -> String {
        match self {
            AST::Null => "".to_string(),
            AST::Integer(x) => x.to_string(),
            AST::String(v) => format!("\"{v}\""),
            AST::Character(c) => format!("\'{c}\'"),
            AST::UnaryExpression { op, child } => match op {
                Operation::Inv => format!("!{}", child.to_string()),
                Operation::Neg => format!("-{}", child.to_string()),
                Operation::Inc => format!("++{}", child.to_string()),
                Operation::Dec => format!("--{}", child.to_string()),
                Operation::Ref => format!("&{}", child.to_string()),
                Operation::Deref => format!("@{}", child.to_string()),
                _ => {
                    eprintln!("{:?} is not a unary operation and therefore the full expression cannot be converted to a string", op);
                    process::exit(1);
                }
            }
            AST::BinaryExpression { op, lhs, rhs } => match op {
                Operation::Add => format!("{} + {}", lhs.to_string(), rhs.to_string()),
                Operation::Sub => format!("{} - {}", lhs.to_string(), rhs.to_string()),
                Operation::Mul => format!("{} * {}", lhs.to_string(), rhs.to_string()),
                Operation::Div => format!("{} / {}", lhs.to_string(), rhs.to_string()),
                Operation::Mod => format!("{} % {}", lhs.to_string(), rhs.to_string()),
                Operation::Or => format!("{} || {}", lhs.to_string(), rhs.to_string()),
                Operation::And => format!("{} && {}", lhs.to_string(), rhs.to_string()),
                Operation::Eq => format!("{} == {}", lhs.to_string(), rhs.to_string()),
                Operation::Neq => format!("{} != {}", lhs.to_string(), rhs.to_string()),
                Operation::LT => format!("{} < {}", lhs.to_string(), rhs.to_string()),
                Operation::GT => format!("{} > {}", lhs.to_string(), rhs.to_string()),
                Operation::GTE => format!("{} >= {}", lhs.to_string(), rhs.to_string()),
                Operation::LTE => format!("{} <= {}", lhs.to_string(), rhs.to_string()),
                _ => {
                    eprintln!("{:?} is not a binary operation and therefore the full expression cannot be converted to a string", op);
                    process::exit(1);
                }
            }
            AST::Argument(v) => v.to_string(),
            AST::Parameter { param_type, name } => format!("{name}: {param_type}"),
            // AST::FunctionDeclaration { name, return_type, prototype, body } => {
            //     let p_str = String::from("THIS A FUNCTION P_STR");
            //     
            //     format!("func {name} :: ({p_str}) {}", body.to_string())
            // }
            AST::FunctionCall { name, arguments } => {
                let mut a_str = String::new();
                if arguments.len() > 0 {
                    a_str.push_str(arguments.first().unwrap().to_string().as_str());
                }
                for arg in arguments.iter().skip(1) {
                    a_str.push_str(", ");
                    a_str.push_str(arg.to_string().as_str());
                }
                format!("{name}({a_str})")
            }
            AST::Block(statements) => {
                let mut b_str = String::from("{\n");
                statements.iter().for_each(|x| b_str.push_str(format!("\t{};\n", x.to_string()).as_str()));
                format!("{b_str}}}")
            }
            AST::VariableCall { name } => name.to_string(),
            _ => todo!("{:#?}", self),
        }
    }
}

pub struct Parser {
    pub token_list: Vec<Token>,
    pub index: usize,
    pub current_token: Token,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Parser {
        Parser {
            token_list: token_list.clone(),
            index: 0,
            current_token: token_list.get(0).unwrap_or(&Token {
                token_type: TokenType::EndOfFile,
                value: '\0'.to_string(),
            }).clone(),
        }
    }
    fn advance(&mut self) {
        if self.index < self.token_list.len() - 1 {
            self.index += 1;
            self.current_token = self.token_list[self.index].clone();
        } else {
            self.current_token = Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
            };
        }
    }
    fn peek(&self, offset: usize) -> Token {
        if self.index + offset >= self.token_list.len() {
            return Token {
                value: String::new(),
                token_type: TokenType::EndOfFile,
            };
        }
        self.token_list[self.index + offset].clone()
    }
    fn eat(&mut self, token_type: TokenType) {
        if self.current_token.token_type != token_type {
            println!(
                "{:?} @ {} = {:?}",
                self.current_token, self.index, self.token_list[self.index]
            );
            eprintln!(
                "[Parser] Expected token of type: {:?}, but instead got token of type: {:?}",
                token_type, self.current_token.token_type
            );
            self.print_token_debug_stack(5);
            process::exit(1);
        }
        self.advance();
    }
    fn print_token_debug_stack(&self, radius: usize) {
        let mut i: usize = 0;
        if radius < self.index {
            i = self.index - radius;
        }
        while i < self.token_list.len() && i <= self.index + radius {
            if i == self.index {
                eprintln!(" -> {:?}", self.token_list[i]);
            } else {
                eprintln!("    {:?}", self.token_list[i]);
            }
            i += 1;
        }
    }
    pub fn parse(&mut self) -> Box<AST> {
        // println!("{:#?}", self.token_list);
        self.scope()
    }
    fn scope(&mut self) -> Box<AST> {
        let mut scope = Box::new(AST::Null);

        let brace_delim: bool = self.current_token.token_type == TokenType::LeftBrace;
        if brace_delim {
            self.eat(TokenType::LeftBrace);
            scope = Box::new(AST::Block(Vec::new()));
        }
        while self.current_token.token_type != TokenType::EndOfFile
            && self.current_token.token_type != TokenType::RightBrace
        {
            // println!("whiling");
            let mut stmt: Option<Box<AST>> = None;
            if self.current_token.token_type == TokenType::SemiColon { continue; }
            if self.current_token.token_type == TokenType::Import {
                self.eat(TokenType::Import);

                let mut mod_path = String::new();
                while self.current_token.token_type != TokenType::SemiColon
                    && self.current_token.token_type != TokenType::EndOfFile
                {
                    mod_path.push_str(self.current_token.value.as_str());
                    self.advance();
                }
                self.eat(TokenType::SemiColon);
                stmt = Some(Box::new(AST::Import { module: mod_path }));
            } else if self.current_token.token_type == TokenType::Function {
                self.eat(TokenType::Function);
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                self.eat(TokenType::LeftParenthesis);
                let mut return_type = String::new();
                while ![TokenType::Comma, TokenType::RightParenthesis].contains(&self.current_token.token_type) {
                    return_type.push_str(self.current_token.value.as_str());
                    self.advance();
                }
                if self.current_token.token_type == TokenType::Comma { self.advance(); }

                let mut prototype = Vec::new(); // really just a vector of parameters, not a full
                                                // prototype
                while self.current_token.token_type != TokenType::RightParenthesis
                    && self.current_token.token_type != TokenType::EndOfFile
                {
                    let param_name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);

                    let mut param_type = String::new();
                    self.eat(TokenType::Colon);
                    while self.current_token.token_type != TokenType::RightParenthesis
                        && self.current_token.token_type != TokenType::Comma
                        && self.current_token.token_type != TokenType::EndOfFile
                    {
                        param_type.push_str(self.current_token.value.as_str());
                        self.advance();
                    }
                    prototype.push(Box::new(AST::Parameter {
                        param_type,
                        name: param_name,
                    }));
                    if self.current_token.token_type == TokenType::Comma {
                        // there is still more to parse
                        self.eat(TokenType::Comma);
                    }
                }
                self.eat(TokenType::RightParenthesis);

                // println!("PARAMS: {:?}", prototype);
                let body = self.scope();

                stmt = Some(Box::new(AST::FunctionDeclaration {
                    name,
                    return_type,
                    prototype,
                    body,
                }));
            } else if self.current_token.token_type == TokenType::Asm {
                self.eat(TokenType::Asm);
                let assembly = self.current_token.value.clone();
                self.eat(TokenType::String);
                self.eat(TokenType::SemiColon);
                stmt = Some(Box::new(AST::Asm(assembly)));
            } else if self.current_token.token_type == TokenType::If {
                self.eat(TokenType::If);
                let condition = self.expression().unwrap();
                let body = self.scope();
                let mut alt = None;
                if self.current_token.token_type == TokenType::Else {
                    self.eat(TokenType::Else);
                    alt = Some(self.scope());
                }
                stmt = Some(Box::new(AST::If {
                    condition,
                    body,
                    alt,
                }));
            } else if self.current_token.token_type == TokenType::Return {
                self.eat(TokenType::Return);
                let value = self.expression().unwrap();
                self.eat(TokenType::SemiColon);
                stmt = Some(Box::new(AST::Return(value)));
            } else if [TokenType::Let, TokenType::Static].contains(&self.current_token.token_type) {
                let is_static = self.current_token.token_type == TokenType::Static;
                self.eat(if is_static { TokenType::Static } else { TokenType::Let });
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);
                self.eat(TokenType::Colon);
                let mut variable_type = String::new();
                while self.current_token.token_type != TokenType::Equals
                    && self.current_token.token_type != TokenType::SemiColon
                {
                    variable_type.push_str(self.current_token.value.as_str());
                    self.advance();
                }
                let set_value = self.current_token.token_type == TokenType::Equals;
                if set_value {
                    self.eat(TokenType::Equals);
                    match *scope {
                        AST::Block(ref mut statements) => {
                            statements.push(Box::new(AST::VariableDeclaration {
                                variable_type,
                                name: name.clone(),
                                is_static,
                            }));
                        }
                        _ => unreachable!(),
                    };

                    stmt = Some(Box::new(AST::VariableAssignment {
                        name,
                        value: self.expression().unwrap(),
                    }));
                } else {
                    stmt = Some(Box::new(AST::VariableDeclaration {
                        variable_type,
                        name: name.clone(),
                        is_static,
                    }));
                }
                self.eat(TokenType::SemiColon);
            } else if self.current_token.token_type == TokenType::For {
                // TODO: implement updater
                self.eat(TokenType::For);
                self.eat(TokenType::LeftParenthesis);
                let initializer = self.scope();
                let condition = self.expression().unwrap();
                self.eat(TokenType::SemiColon);
                let updater = self.parse();
                self.eat(TokenType::RightParenthesis);
                let body = self.scope();

                stmt = Some(Box::new(AST::For {
                    initializer,
                    condition,
                    updater,
                    body,
                }));
            } else if self.current_token.token_type == TokenType::While {
                self.eat(TokenType::While);
                self.eat(TokenType::LeftParenthesis);
                let condition = self.expression().unwrap();
                self.eat(TokenType::RightParenthesis);
                let body = self.scope();

                stmt = Some(Box::new(AST::While { condition, body }));
            } else if self.current_token.token_type == TokenType::Struct {
                self.eat(TokenType::Struct);
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                self.eat(TokenType::LeftBrace);
                let mut members = Vec::new();
                while self.current_token.token_type != TokenType::RightBrace {
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::Colon);

                    let mut sub = String::new();
                    while self.current_token.token_type != TokenType::SemiColon {
                        sub.push_str(self.current_token.value.as_str());
                        self.advance();
                    }
                    self.eat(TokenType::SemiColon);
                    members.push((name, sub));
                }
                self.eat(TokenType::RightBrace);

                stmt = Some(Box::new(AST::Struct { name, members }));
            } else if self.current_token.token_type == TokenType::Enum {
                self.eat(TokenType::Enum);
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                let mut attributes = Vec::new();
                if self.current_token.token_type == TokenType::LeftBracket {
                    // these cool new things called attributes
                    self.eat(TokenType::LeftBracket);
                    while self.current_token.token_type != TokenType::RightBracket {
                        let name = self.current_token.value.clone();
                        attributes.push(name);
                        self.eat(TokenType::Identifier);
                        if self.current_token.token_type == TokenType::Comma {
                            self.eat(TokenType::Comma);
                        } else { break; }
                    }
                    self.eat(TokenType::RightBracket);
                }

                self.eat(TokenType::LeftBrace);
                let mut values = Vec::new();
                while self.current_token.token_type != TokenType::RightBrace {
                    values.push(self.current_token.value.clone());
                    self.eat(TokenType::Identifier);
                    if self.current_token.token_type == TokenType::Comma {
                        self.eat(TokenType::Comma);
                    } else { break; }
                }
                self.eat(TokenType::RightBrace);

                stmt = Some(Box::new(AST::Enum { name, values, attributes, }));
            } else if self.current_token.token_type == TokenType::Def {
                // definition
                self.eat(TokenType::Def);

                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                let value = self.expression().unwrap();
                self.eat(TokenType::SemiColon);

                stmt = Some(Box::new(AST::Definition { name, value }));
            } else if self.current_token.token_type == TokenType::TypeDef {
                // type definition
                self.eat(TokenType::TypeDef);

                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                let mut type_string = String::new();
                while self.current_token.token_type != TokenType::SemiColon {
                    type_string.push_str(self.current_token.value.as_str());
                    self.advance();
                }
                self.eat(TokenType::SemiColon);

                stmt = Some(Box::new(AST::TypeDefinition { name, type_string }))
            } else if self.current_token.token_type == TokenType::Macro {
                // macro
                self.eat(TokenType::Macro);

                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);

                self.eat(TokenType::LeftParenthesis);
                let mut parameters = Vec::new();
                while self.current_token.token_type != TokenType::RightParenthesis {
                    parameters.push(self.current_token.value.clone());
                    self.advance();
                    if self.current_token.token_type == TokenType::Comma { self.eat(TokenType::Comma); } else { break; }
                }
                self.eat(TokenType::RightParenthesis);

                let plaintext_expansion = self.scope().to_string();
                let tokens = Tokenizer::new(SourceFile { path: "".to_string(), contents: plaintext_expansion }).tokenize();
                let mut expansion = Vec::new(); // (IS_TEMPLATE, VALUE)
                let mut string_buffer = String::new();
                for token in tokens {
                    if token.token_type == TokenType::Identifier && parameters.contains(&token.value) {
                        if string_buffer.len() > 0 { expansion.push((false, string_buffer.clone())); }
                        string_buffer.clear();

                        expansion.push((true, token.value));
                    } else {
                        string_buffer.push_str(token.value.as_str());
                    }
                }
                if string_buffer.len() > 0 { expansion.push((false, string_buffer)); }

                stmt = Some(Box::new(AST::Macro { name, parameters, expansion }));
            } else if self.current_token.token_type == TokenType::Identifier {
                if self.peek(1).token_type == TokenType::LeftParenthesis {
                    // function call
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::LeftParenthesis);

                    let mut arguments = Vec::new();
                    while self.current_token.token_type != TokenType::RightParenthesis
                        && self.current_token.token_type != TokenType::EndOfFile
                    {
                        arguments.push(Box::new(AST::Argument(self.expression().unwrap())));
                        if self.current_token.token_type == TokenType::Comma {
                            // there is still more to parse
                            self.eat(TokenType::Comma);
                        }
                    }
                    self.eat(TokenType::RightParenthesis);

                    self.eat(TokenType::SemiColon);

                    stmt = Some(Box::new(AST::FunctionCall { name, arguments }));
                } /*else {
                      // variable assignment
                      let name = self.current_token.value.clone();
                      self.eat(TokenType::Identifier);
                      self.eat(TokenType::Equals);

                      let value = self.expression().unwrap();
                      self.eat(TokenType::SemiColon);

                      stmt = Some(Box::new(AST::VariableAssignment { name, value }));
                  }*/
            }
            if let Some(s) = stmt {
                match *scope {
                    AST::Block(ref mut statements) => {
                        statements.push(s);
                    }
                    AST::Null => {
                        scope = s;
                    }
                    _ => unreachable!(),
                }
            } else {
                // check if it happens to be a statement
                let expr = self.expression();
                stmt = match expr {
                    // _ => None,
                    Ok(ast) => Some(ast),
                    Err(_) => {
                        let radius = 4;
                        eprintln!(
                            "[Parser] Cannot find matching parse method for tokens in order (radius = {radius}):"
                        );
                        self.print_token_debug_stack(radius);
                        process::exit(1);
                    }
                };
                match *scope {
                    AST::Block(ref mut statements) => {
                        statements.push(stmt.unwrap());
                    }
                    _ => unreachable!(),
                }
                self.eat(TokenType::SemiColon);
            }
            if !brace_delim {
                break;
            }
        }
        if brace_delim {
            self.eat(TokenType::RightBrace);
        }
        scope
    }
    fn expression(&mut self) -> Result<Box<AST>, String> {
        // println!("{:?}", self.current_token);
        let mut lhs = self.assignment()?;

        while self.current_token.token_type == TokenType::DoublePipe
            || self.current_token.token_type == TokenType::DoubleAmpersand
        {
            // println!("expression");
            let op = match self.current_token.token_type {
                TokenType::DoublePipe => Ok(Operation::Or),
                TokenType::DoubleAmpersand => Ok(Operation::And),
                TokenType::Plus => Ok(Operation::Add),
                _ => {
                    // eprintln!(
                    //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                    //     self.current_token.token_type
                    // );
                    // process::exit(1);
                    Err(
                        format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type)
                    )
                }
            }?;
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.assignment()?,
            });
        }

        Ok(lhs)
    }
    fn assignment(&mut self) -> Result<Box<AST>, String> {
        let mut lhs = self.comparison()?;

        while self.current_token.token_type == TokenType::Equals {
            let op = match self.current_token.token_type {
                TokenType::Equals => Ok(Operation::Assign),
                _ => {
                    // eprintln!("[ExpressionParser] {:?} is not a valid operation, or is has not been implemented yet", self.current_token.token_type);
                    // process::exit(1);
                    Err(format!("[ExpressionParser] {:?} is not a valid operation, or is has not been implemented yet", self.current_token.token_type))
                }
            }?;
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.comparison()?,
            });
        }

        Ok(lhs)
    }
    fn comparison(&mut self) -> Result<Box<AST>, String> {
        // println!("{:?}", self.current_token.token_type);
        let mut lhs = self.term()?;

        while self.current_token.token_type == TokenType::EqualsTo
            || self.current_token.token_type == TokenType::NotEqualsTo
            || self.current_token.token_type == TokenType::LessThan
            || self.current_token.token_type == TokenType::MoreThan
            || self.current_token.token_type == TokenType::LessThanEqualsTo
            || self.current_token.token_type == TokenType::MoreThanEqualsTo
        {
            // println!("comparison");
            let op = match self.current_token.token_type {
                TokenType::EqualsTo => Ok(Operation::Eq),
                TokenType::NotEqualsTo => Ok(Operation::Neq),
                TokenType::LessThan => Ok(Operation::LT),
                TokenType::MoreThan => Ok(Operation::GT),
                TokenType::LessThanEqualsTo => Ok(Operation::LTE),
                TokenType::MoreThanEqualsTo => Ok(Operation::GTE),
                _ => {
                    // eprintln!(
                    //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                    //     self.current_token.token_type
                    // );
                    // process::exit(1);
                    Err(format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet", self.current_token.token_type))
                }
            }?;
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.term()?,
            });
        }

        Ok(lhs)
    }
    fn term(&mut self) -> Result<Box<AST>, String> {
        let mut lhs = self.factor()?;

        while self.current_token.token_type == TokenType::Plus
            || self.current_token.token_type == TokenType::Minus
        {
            // println!("term");
            let op = match self.current_token.token_type {
                TokenType::Plus => Ok(Operation::Add),
                TokenType::Minus => Ok(Operation::Sub),
                _ => {
                    // eprintln!(
                    //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                    //     self.current_token.token_type
                    // );
                    // process::exit(1);
                    Err(format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet", self.current_token.token_type))
                }
            }?;
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.factor()?,
            });
        }

        Ok(lhs)
    }
    fn factor(&mut self) -> Result<Box<AST>, String> {
        let mut lhs = self.unary()?;

        while self.current_token.token_type == TokenType::Star
            || self.current_token.token_type == TokenType::Slash
            || self.current_token.token_type == TokenType::Percent
        {
            // println!("factor");
            let op = match self.current_token.token_type {
                TokenType::Star => Ok(Operation::Mul),
                TokenType::Slash => Ok(Operation::Div),
                TokenType::Percent => Ok(Operation::Mod),
                _ => {
                    // eprintln!(
                    //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                    //     self.current_token.token_type
                    // );
                    // process::exit(1);
                    Err(format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet", self.current_token.token_type))
                }
            }?;
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.unary()?,
            })
        }

        Ok(lhs)
    }
    fn unary(&mut self) -> Result<Box<AST>, String> {
        if self.current_token.token_type == TokenType::Bang
            || self.current_token.token_type == TokenType::Minus
            || self.current_token.token_type == TokenType::Increment
            || self.current_token.token_type == TokenType::Decrement
            || self.current_token.token_type == TokenType::Ampersand
            || self.current_token.token_type == TokenType::At
        {
            // println!("unary");
            let op = match self.current_token.token_type {
                TokenType::Minus => Ok(Operation::Neg),
                TokenType::Bang => Ok(Operation::Inv),
                TokenType::Increment => Ok(Operation::Inc),
                TokenType::Decrement => Ok(Operation::Dec),
                TokenType::Ampersand => Ok(Operation::Ref),
                TokenType::At => Ok(Operation::Deref),
                _ => {
                    // eprintln!(
                    //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                    //     self.current_token.token_type
                    // );
                    // process::exit(1);
                    Err(format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet", self.current_token.token_type))
                }
            }?;
            self.advance();
            return Ok(Box::new(AST::UnaryExpression {
                op,
                child: self.accessor()?,
            }));
        }
        self.accessor()
    }
    fn accessor(&mut self) -> Result<Box<AST>, String> {
        // should be used for '->', '[]', '.', '::' style operators
        let mut lhs = self.primary()?;

        if self.current_token.token_type == TokenType::Period {
            self.eat(TokenType::Period);
            lhs = Box::new(AST::MemberAccess {
                accessed: lhs,
                member: self.current_token.value.clone(),
            });
            self.eat(TokenType::Identifier);
        } else if self.current_token.token_type == TokenType::BlockSeparator {
            self.eat(TokenType::BlockSeparator);
            let parent = match *lhs {
                AST::VariableCall { name } => name,
                _ => unreachable!(),
            };
            let child = self.current_token.value.clone();
            self.eat(TokenType::Identifier);
            lhs = Box::new(AST::VariableCall { name: format!("{parent}::{child}") });
        } else {
            while self.current_token.token_type == TokenType::LeftBracket
                || self.current_token.token_type == TokenType::Period
            {
                // match self.current_token.token_type {
                //     TokenType::LeftBracket
                // }
                let mut rhs = None;
                let op = match self.current_token.token_type {
                    TokenType::LeftBracket => {
                        self.eat(TokenType::LeftBracket);
                        rhs = Some(self.expression()?);
                        self.eat(TokenType::RightBracket);
                        Ok(Operation::ArrAcc)
                    }
                    _ => {
                        // eprintln!(
                        //     "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        //     self.current_token.token_type
                        // );
                        // process::exit(1);
                        Err(format!("[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet", self.current_token.token_type))
                    }
                }?;
                lhs = Box::new(AST::BinaryExpression {
                    op,
                    lhs,
                    rhs: rhs.unwrap(),
                });
            }
        }

        Ok(lhs)
    }
    fn primary(&mut self) -> Result<Box<AST>, String> {
        // println!("primary");
        match self.current_token.token_type {
            TokenType::Integer => {
                // println!("integer");
                let value = self
                    .current_token
                    .value
                    .as_str()
                    .parse::<i64>()
                    .unwrap_or(0);
                self.eat(TokenType::Integer);
                Ok(Box::new(AST::Integer(value)))
            }
            TokenType::String => {
                // println!("string");
                let value = self.current_token.value.clone();
                self.eat(TokenType::String);
                Ok(Box::new(AST::String(value)))
            }
            TokenType::Character => {
                let value = self.current_token.value.chars().nth(0).unwrap_or('\0');
                self.eat(TokenType::Character);
                Ok(Box::new(AST::Character(value)))
            }
            TokenType::Identifier => {
                if self.peek(1).token_type == TokenType::LeftParenthesis {
                    // println!("function call");
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::LeftParenthesis);
                    let mut arguments = Vec::new();
                    while self.current_token.token_type != TokenType::RightParenthesis {
                        let value = self.expression()?;
                        arguments.push(Box::new(AST::Argument(value)));
                        if self.current_token.token_type != TokenType::RightParenthesis {
                            self.eat(TokenType::Comma);
                        }
                    }
                    self.eat(TokenType::RightParenthesis);
                    return Ok(Box::new(AST::FunctionCall { name, arguments }));
                } else if self.peek(1).token_type == TokenType::LeftBrace {
                    // println!("struct initializer");
                    // self.print_token_debug_stack(4);
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::LeftBrace);
                    let initializer = if self.current_token.token_type == TokenType::Identifier
                        && self.peek(1).token_type == TokenType::Colon
                    {
                        // yep, its a full initialization
                        let mut members = Vec::new();
                        while self.current_token.token_type != TokenType::RightBrace {
                            let member_name = self.current_token.value.clone();
                            self.eat(TokenType::Identifier);
                            if self.current_token.token_type != TokenType::Colon {
                                break;
                            }
                            self.eat(TokenType::Colon);
                            let member_value = self.expression()?;
                            self.eat(TokenType::Comma);
                            members.push((member_name, member_value));
                        }
                        Box::new(AST::StructInitializer {
                            spreads: false,
                            name,
                            members,
                        })
                    } else {
                        // single expression fill
                        let value = self.expression()?;
                        Box::new(AST::StructInitializer {
                            spreads: true,
                            name,
                            members: vec![("".to_string(), value)],
                        })
                    };
                    self.eat(TokenType::RightBrace);
                    return Ok(initializer);
                }
                // println!("variable call");
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);
                Ok(Box::new(AST::VariableCall { name }))
            }
            TokenType::LeftParenthesis => {
                // println!("grouping");
                self.eat(TokenType::LeftParenthesis);
                let group = self.expression()?;
                self.eat(TokenType::RightParenthesis);
                Ok(group)
            }
            _ => self.expression(),
        }
    }
}
