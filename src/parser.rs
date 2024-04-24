use std::{process, usize};

use crate::lexer::{Token, TokenType};

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
}

#[derive(Debug, Clone)]
pub enum AST {
    Integer(i64), // TODO: maybe implement 64 bits???
    String(String),
    UnaryExpr {
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
            current_token: token_list[0].clone(),
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
                eprintln!(" > {:?}", self.token_list[i]);
            } else {
                eprintln!("   {:?}", self.token_list[i]);
            }
            i += 1;
        }
    }
    pub fn parse(&mut self) -> Box<AST> {
        // println!("{:#?}", self.token_list);
        self.scope()
    }
    fn scope(&mut self) -> Box<AST> {
        let mut scope = Box::new(AST::Block(Vec::new()));

        let brace_delim: bool = self.current_token.token_type == TokenType::LeftBrace;
        if brace_delim {
            self.eat(TokenType::LeftBrace);
        }
        while self.current_token.token_type != TokenType::EndOfFile
            && self.current_token.token_type != TokenType::RightBrace
        {
            // println!("whiling");
            let mut stmt: Option<Box<AST>> = None;
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

                self.eat(TokenType::Colon);
                let mut return_type = String::new();
                while self.current_token.token_type != TokenType::Equals {
                    return_type.push_str(self.current_token.value.as_str());
                    self.advance();
                }

                self.eat(TokenType::Equals);

                self.eat(TokenType::LeftParenthesis);
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
                let condition = self.expression();
                let body = self.scope();
                stmt = Some(Box::new(AST::If { condition, body }));
            } else if self.current_token.token_type == TokenType::Return {
                self.eat(TokenType::Return);
                let value = self.expression();
                self.eat(TokenType::SemiColon);
                stmt = Some(Box::new(AST::Return(value)));
            } else if self.current_token.token_type == TokenType::Let {
                self.eat(TokenType::Let);
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
                            }));
                        }
                        _ => unreachable!(),
                    };

                    stmt = Some(Box::new(AST::VariableAssignment {
                        name,
                        value: self.expression(),
                    }));
                } else {
                    stmt = Some(Box::new(AST::VariableDeclaration {
                        variable_type,
                        name: name.clone(),
                    }));
                }
                self.eat(TokenType::SemiColon);
            } else if self.current_token.token_type == TokenType::For {
                // TODO: implement updater
                self.eat(TokenType::For);
                self.eat(TokenType::LeftParenthesis);
                let initializer = self.scope();
                let condition = self.expression();
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
                let condition = self.expression();
                self.eat(TokenType::RightParenthesis);
                let body = self.scope();

                stmt = Some(Box::new(AST::While { condition, body }));
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
                        arguments.push(Box::new(AST::Argument(self.expression())));
                        if self.current_token.token_type == TokenType::Comma {
                            // there is still more to parse
                            self.eat(TokenType::Comma);
                        }
                    }
                    self.eat(TokenType::RightParenthesis);

                    self.eat(TokenType::SemiColon);

                    stmt = Some(Box::new(AST::FunctionCall { name, arguments }));
                } else {
                    // variable assignment
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::Equals);

                    let value = self.expression();
                    self.eat(TokenType::SemiColon);

                    stmt = Some(Box::new(AST::VariableAssignment { name, value }));
                }
            }
            if let Some(s) = stmt {
                match *scope {
                    AST::Block(ref mut statements) => {
                        statements.push(s);
                    }
                    _ => unreachable!(),
                }
            } else {
                // println!("{:#?}", scope);

                let radius = 4;
                eprintln!(
                    "[Parser] Cannot find matching parse method for tokens in order (radius = {radius}):"
                );
                self.print_token_debug_stack(radius);
                process::exit(1);
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
    fn expression(&mut self) -> Box<AST> {
        // println!("{:?}", self.current_token);
        let mut lhs = self.comparison();

        while self.current_token.token_type == TokenType::DoublePipe
            || self.current_token.token_type == TokenType::DoubleAmpersand
        {
            // println!("expression");
            let op = match self.current_token.token_type {
                TokenType::DoublePipe => Operation::Or,
                TokenType::DoubleAmpersand => Operation::And,
                TokenType::Plus => Operation::Add,
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.comparison(),
            });
        }

        lhs
    }
    fn comparison(&mut self) -> Box<AST> {
        // println!("{:?}", self.current_token.token_type);
        let mut lhs = self.term();

        while self.current_token.token_type == TokenType::EqualsTo
            || self.current_token.token_type == TokenType::NotEqualsTo
            || self.current_token.token_type == TokenType::LessThan
            || self.current_token.token_type == TokenType::MoreThan
            || self.current_token.token_type == TokenType::LessThanEqualsTo
            || self.current_token.token_type == TokenType::MoreThanEqualsTo
        {
            // println!("comparison");
            let op = match self.current_token.token_type {
                TokenType::EqualsTo => Operation::Eq,
                TokenType::NotEqualsTo => Operation::Neq,
                TokenType::LessThan => Operation::LT,
                TokenType::MoreThan => Operation::GT,
                TokenType::LessThanEqualsTo => Operation::LTE,
                TokenType::MoreThanEqualsTo => Operation::GTE,
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.term(),
            });
        }

        lhs
    }
    fn term(&mut self) -> Box<AST> {
        let mut lhs = self.factor();

        while self.current_token.token_type == TokenType::Plus
            || self.current_token.token_type == TokenType::Minus
        {
            // println!("term");
            let op = match self.current_token.token_type {
                TokenType::Plus => Operation::Add,
                TokenType::Minus => Operation::Sub,
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.factor(),
            });
        }

        lhs
    }
    fn factor(&mut self) -> Box<AST> {
        let mut lhs = self.unary();

        while self.current_token.token_type == TokenType::Star
            || self.current_token.token_type == TokenType::Slash
            || self.current_token.token_type == TokenType::Percent
        {
            // println!("factor");
            let op = match self.current_token.token_type {
                TokenType::Star => Operation::Mul,
                TokenType::Slash => Operation::Div,
                TokenType::Percent => Operation::Mod,
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            self.advance();
            lhs = Box::new(AST::BinaryExpression {
                op,
                lhs,
                rhs: self.unary(),
            })
        }

        lhs
    }
    fn unary(&mut self) -> Box<AST> {
        if self.current_token.token_type == TokenType::Bang
            || self.current_token.token_type == TokenType::Minus
            || self.current_token.token_type == TokenType::Increment
            || self.current_token.token_type == TokenType::Decrement
        {
            // println!("unary");
            let op = match self.current_token.token_type {
                TokenType::Minus => Operation::Neg,
                TokenType::Bang => Operation::Inv,
                TokenType::Increment => Operation::Inc,
                TokenType::Decrement => Operation::Dec,
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            self.advance();
            return Box::new(AST::UnaryExpr {
                op,
                child: self.accessor(),
            });
        }
        self.accessor()
    }
    fn accessor(&mut self) -> Box<AST> {
        // should be used for '->', '[]', '.' type operators
        let mut lhs = self.primary();

        while self.current_token.token_type == TokenType::LeftBracket {
            let rhs;
            let op = match self.current_token.token_type {
                TokenType::LeftBracket => {
                    self.eat(TokenType::LeftBracket);
                    rhs = self.expression();
                    self.eat(TokenType::RightBracket);
                    Operation::ArrAcc
                }
                _ => {
                    eprintln!(
                        "[ExpressionParser] {:?} is not a valid operation, or it has not been implemented yet",
                        self.current_token.token_type
                    );
                    process::exit(1);
                }
            };
            lhs = Box::new(AST::BinaryExpression { op, lhs, rhs });
        }

        lhs
    }
    fn primary(&mut self) -> Box<AST> {
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
                Box::new(AST::Integer(value))
            }
            TokenType::String => {
                // println!("string");
                let value = self.current_token.value.clone();
                self.eat(TokenType::String);
                Box::new(AST::String(value))
            }
            TokenType::Identifier => {
                if self.peek(1).token_type == TokenType::LeftParenthesis {
                    // println!("function call");
                    let name = self.current_token.value.clone();
                    self.eat(TokenType::Identifier);
                    self.eat(TokenType::LeftParenthesis);
                    let mut arguments = Vec::new();
                    while self.current_token.token_type != TokenType::RightParenthesis {
                        let value = self.expression();
                        arguments.push(Box::new(AST::Argument(value)));
                        if self.current_token.token_type != TokenType::RightParenthesis {
                            self.eat(TokenType::Comma);
                        }
                    }
                    self.eat(TokenType::RightParenthesis);
                    return Box::new(AST::FunctionCall { name, arguments });
                }
                // println!("variable call");
                let name = self.current_token.value.clone();
                self.eat(TokenType::Identifier);
                Box::new(AST::VariableCall { name })
            }
            TokenType::LeftParenthesis => {
                // println!("grouping");
                self.eat(TokenType::LeftParenthesis);
                let group = self.expression();
                self.eat(TokenType::RightParenthesis);
                group
            }
            _ => {
                // println!("something else ({:?})", self.peek(1));
                self.expression()
            }
        }
    }
}
