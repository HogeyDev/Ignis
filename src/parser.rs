use std::{process, usize};

use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum Operation {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    Inv, // !
    Neg, // -

    Or,  // ||
    And, // &&
}

#[derive(Debug)]
pub enum AST {
    Integer(i32), // TODO: maybe implement 64 bits???
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
            eprintln!(
                "Expected token of type: {:?}, but instead got token of type: {:?}",
                token_type, self.current_token.token_type
            );
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
            && self.current_token.token_type != TokenType::EndOfFile
        {
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
            } else if self.current_token.token_type == TokenType::If {
                self.eat(TokenType::If);
                let condition = self.expression();
                let body = self.scope();
                stmt = Some(Box::new(AST::If { condition, body }));
            }
            if let Some(s) = stmt {
                match *scope {
                    AST::Block(ref mut statements) => {
                        statements.push(s);
                    }
                    _ => unreachable!(),
                }
            } else {
                println!("{:#?}", scope);

                let radius = 2;
                eprintln!(
                    "Cannot find matching parse method for tokens in order (radius = {radius}):"
                );
                self.print_token_debug_stack(radius);
                process::exit(1);
            }
        }
        scope
    }
    fn expression(&mut self) -> Box<AST> {
        // TODO: Implement expression parsing
        let mut lhs = self.comparison();

        while self.current_token.token_type == TokenType::DoublePipe
            || self.current_token.token_type == TokenType::DoubleAmpersand
        {
            let op = match self.current_token.token_type {
                TokenType::DoublePipe => Operation::Or,
                TokenType::DoubleAmpersand => Operation::And,
                TokenType::Plus => Operation::Add,
                _ => {
                    eprintln!(
                        "{:?} is not a valid operation, or it has not been implemented yet",
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
        let mut lhs = self.term();

        while self.current_token.token_type == TokenType::EqualsTo
            || self.current_token.token_type == TokenType::NotEqualsTo
            || self.current_token.token_type == TokenType::LessThan
            || self.current_token.token_type == TokenType::MoreThan
            || self.current_token.token_type == TokenType::LessThanEqualsTo
            || self.current_token.token_type == TokenType::MoreThanEqualsTo
        {
            let op = match self.current_token.token_type {
                TokenType::
            }
        }

        lhs
    }
}
