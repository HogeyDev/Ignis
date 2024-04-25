use std::process;

use crate::{
    parser::{Operation, AST},
    scope::ScopeContext,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Primative(String),
    Array(Box<Type>),
    Pointer(Box<Type>),
    UnaryOperation(Operation, Box<Type>),
    BinaryOperation(Operation, Box<Type>, Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        todo!("Implement Type::to_string()");
    }
}

pub fn calculate_expression_type(
    ast: Box<AST>,
    scope: &ScopeContext,
) -> Result<Box<Type>, &'static str> {
    let tree = ast_to_type_tree(ast, scope)?;
    let collapsed = collapse_type_tree(tree)?;
    Ok(collapsed.clone())
}

pub fn is_primative_type(potential: String) -> bool {
    vec!["int", "char"].contains(&potential.as_str())
}

pub fn get_primative_type_size(prim: String) -> Result<usize, &'static str> {
    match prim.as_str() {
        "int" => Ok(8),
        "char" => Ok(1),
        _ => Err("Not a primative type"),
    }
}

pub fn get_type_size(comp: Box<Type>) -> Result<usize, &'static str> {
    match *comp {
        Type::Primative(prim) => get_primative_type_size(prim),
        Type::Pointer(sub) => get_type_size(sub),
        Type::UnaryOperation(_, sub) => get_type_size(sub),
        Type::BinaryOperation(_, lhs, _) => get_type_size(lhs),
        _ => {
            eprintln!(
                "[TypeParser] Size of type `{:?}` cannot be inferred at compile-time",
                *comp
            );
            process::exit(1);
        }
    }
}

pub fn ast_to_type_tree(ast: Box<AST>, scope: &ScopeContext) -> Result<Box<Type>, &'static str> {
    match *ast {
        AST::Integer(_) => Ok(Box::new(Type::Primative("int".to_string()))),
        AST::String(_) => Ok(Box::new(Type::Primative("[]char".to_string()))),
        AST::UnaryExpr { op, child } => {
            let child_type = ast_to_type_tree(child, scope)?;
            Ok(Box::new(Type::UnaryOperation(op, child_type)))
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            let lhs_type = ast_to_type_tree(lhs, scope)?;
            let rhs_type = ast_to_type_tree(rhs, scope)?;
            Ok(Box::new(Type::BinaryOperation(op, lhs_type, rhs_type)))
        }
        AST::VariableCall { name } => {
            let type_str = scope.get_variable_data(name).0;
            let variable_type_tree = string_to_type_tree(type_str).unwrap();
            Ok(variable_type_tree)
        }
        _ => {
            eprintln!("[TypeParser] {:?}", ast);
            Err("AST is not type-able")
        }
    }
}

pub fn collapse_type_tree(tree: Box<Type>) -> Result<Box<Type>, &'static str> {
    match *tree {
        Type::Primative(type_name) => Ok(Box::new(Type::Primative(type_name))),
        Type::UnaryOperation(_, child) => collapse_type_tree(child),
        Type::BinaryOperation(_, lhs, rhs) => {
            let collapsed_lhs = collapse_type_tree(lhs)?;
            let collapsed_rhs = collapse_type_tree(rhs)?;

            if collapsed_lhs != collapsed_rhs {
                return Err("Types do not match");
            }
            Ok(collapsed_lhs)
        }
        Type::Pointer(sub_type) => Ok(Box::new(Type::Pointer(collapse_type_tree(sub_type)?))),
        Type::Array(sub_type) => Ok(Box::new(Type::Array(collapse_type_tree(sub_type)?))),
    }
}

pub fn string_to_type_tree(type_str: String) -> Result<Box<Type>, &'static str> {
    let type_tokens = TypeLexer::new(type_str).parse();
    TypeParser::new(type_tokens).parse()
}

#[derive(Debug, Clone, PartialEq)]
enum StrTokType {
    AtSign,
    LeftBracket,
    RightBracket,
    Identifier(String),
}

struct TypeLexer {
    type_string: String,
    index: usize,
    current_char: char,
}

impl TypeLexer {
    pub fn new(type_string: String) -> TypeLexer {
        TypeLexer {
            type_string: type_string.clone(),
            index: 0,
            current_char: type_string.bytes().nth(0).unwrap_or(0).into(),
        }
    }
    fn get_char(&self, index: usize) -> char {
        self.type_string
            .bytes()
            .nth(index)
            .unwrap_or(0)
            .try_into()
            .unwrap()
    }
    fn peek(&self, offset: i64) -> char {
        self.get_char((self.index as i64 + offset).try_into().unwrap())
    }
    fn advance(&mut self) {
        self.current_char = self.peek(1);
        self.index += 1;
    }
    pub fn parse(&mut self) -> Vec<StrTokType> {
        let mut token_list = Vec::new();
        while self.index < self.type_string.len() {
            if self.current_char.is_alphabetic() {
                let mut full_id = String::new();
                while self.current_char.is_alphanumeric() {
                    full_id.push(self.current_char);
                    self.advance();
                }
                token_list.push(StrTokType::Identifier(full_id));
            } else {
                token_list.push(match self.current_char {
                    '@' => StrTokType::AtSign,
                    '[' => StrTokType::LeftBracket,
                    ']' => StrTokType::RightBracket,
                    _ => {
                        eprintln!(
                            "[TypeParser] Character `{}` is not parseable in a type",
                            self.current_char
                        );
                        process::exit(1);
                    }
                });
                self.advance();
            }
        }
        token_list
    }
}

struct TypeParser {
    pub tokens: Vec<StrTokType>,
    pub index: usize,
    pub current_token: StrTokType,
}

impl TypeParser {
    pub fn new(tokens: Vec<StrTokType>) -> TypeParser {
        TypeParser {
            tokens: tokens.clone(),
            index: 0,
            current_token: tokens[0].clone(),
        }
    }
    fn peek(&self, offset: i64) -> StrTokType {
        let new_index: usize = (self.index as i64 + offset)
            .try_into()
            .unwrap_or(self.tokens.len());
        self.tokens[new_index].clone()
    }
    fn advance(&mut self) {
        self.current_token = self.peek(1);
        self.index += 1;
    }
    pub fn parse(&mut self) -> Result<Box<Type>, &'static str> {
        match self.current_token.clone() {
            StrTokType::AtSign => {
                self.advance();
                let pointer_type = self.parse().unwrap();
                let pointer = Type::Pointer(pointer_type);
                Ok(Box::new(pointer))
            }
            StrTokType::LeftBracket => {
                self.advance();
                if self.current_token == StrTokType::RightBracket {
                    self.advance();
                } else {
                    eprintln!(
                        "[TypeParser] Expected a RightBracket, but recieved {:?}",
                        self.current_token
                    );
                    process::exit(1);
                }
                Ok(Box::new(Type::Array(self.parse().unwrap())))
            }
            StrTokType::RightBracket => {
                eprintln!("[TypeParser] Found a random RightBracket");
                process::exit(1);
            }
            StrTokType::Identifier(id) => match id.as_str() {
                "int" => Ok(Box::new(Type::Primative("int".to_string()))),
                "char" => Ok(Box::new(Type::Primative("char".to_string()))),
                _ => {
                    eprintln!("[TypeParser] Resolving complex types does not exist yet! Are you in the future?");
                    process::exit(1);
                }
            },
        }
    }
}
