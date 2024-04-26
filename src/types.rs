use std::process;

use crate::{
    parser::{Operation, AST},
    scope::ScopeContext,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Primative(String),
    DynamicArray(Box<Type>),
    FixedArray(usize, Box<Type>),
    Pointer(Box<Type>),
    UnaryOperation(Operation, Box<Type>),
    BinaryOperation(Operation, Box<Type>, Box<Type>),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self.clone() {
            Self::Primative(id) => id,
            Self::FixedArray(size, sub) => format!("[{}]{}", size, sub.to_string()),
            Self::DynamicArray(sub) => format!("[]{}", sub.to_string()),
            Self::Pointer(sub) => format!("@{}", sub.to_string()),
            _ => {
                eprintln!("[TypeParser] Cannot stringify type `{:?}`", self);
                process::exit(1);
            }
        }
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
        "usize" => Ok(8),
        _ => Err("Not a primative type"),
    }
}

pub fn get_type_size(comp: Box<Type>) -> Result<usize, &'static str> {
    match *comp {
        Type::Primative(prim) => get_primative_type_size(prim),
        Type::Pointer(_) => get_primative_type_size("usize".to_string()),
        Type::UnaryOperation(_, sub) => get_type_size(sub),
        Type::BinaryOperation(_, lhs, _) => get_type_size(lhs),
        Type::FixedArray(size, sub) => Ok(get_type_size(sub).unwrap() * size),
        Type::DynamicArray(_) => get_primative_type_size("usize".to_string()),
        // _ => {
        //     eprintln!(
        //         "[TypeParser] Size of type `{:?}` cannot be inferred at compile-time",
        //         *comp
        //     );
        //     process::exit(1);
        // }
    }
}

pub fn ast_to_type_tree(ast: Box<AST>, scope: &ScopeContext) -> Result<Box<Type>, &'static str> {
    match *ast {
        AST::Integer(_) => Ok(Box::new(Type::Primative("int".to_string()))),
        AST::String(_) => string_to_collapsed_type_tree("[]char".to_string()),
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
        AST::FunctionCall { name, .. } => {
            let type_str = scope.get_function_data(name).0;
            let function_type_tree = string_to_type_tree(type_str).unwrap();
            Ok(function_type_tree)
        }
        AST::Argument(sub) => ast_to_type_tree(sub, scope),
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
        Type::BinaryOperation(op, lhs, rhs) => {
            let collapsed_lhs = collapse_type_tree(lhs.clone())?;
            let collapsed_rhs = collapse_type_tree(rhs)?;

            if op == Operation::ArrAcc {
                return match *lhs {
                    Type::FixedArray(_, sub) => Ok(sub),
                    Type::DynamicArray(sub) => Ok(sub),
                    _ => Err("Tried to do array access on non array type"),
                };
            } else if collapsed_lhs != collapsed_rhs {
                return Err("Types do not match");
            }
            Ok(collapsed_lhs)
        }
        Type::Pointer(sub_type) => Ok(Box::new(Type::Pointer(collapse_type_tree(sub_type)?))),
        Type::DynamicArray(sub_type) => {
            Ok(Box::new(Type::DynamicArray(collapse_type_tree(sub_type)?)))
        }
        Type::FixedArray(size, sub_type) => Ok(Box::new(Type::FixedArray(
            size,
            collapse_type_tree(sub_type)?,
        ))),
    }
}

pub fn string_to_type_tree(type_str: String) -> Result<Box<Type>, &'static str> {
    let type_tokens = TypeLexer::new(type_str).parse();
    TypeParser::new(type_tokens).parse()
}

pub fn string_to_collapsed_type_tree(type_str: String) -> Result<Box<Type>, &'static str> {
    collapse_type_tree(string_to_type_tree(type_str)?)
}

#[derive(Debug, Clone, PartialEq)]
enum StrTokType {
    AtSign,
    LeftBracket,
    RightBracket,
    Identifier(String),
    Integer(usize),
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
            } else if self.current_char.is_numeric() {
                let mut full_number = String::new();
                while self.current_char.is_numeric() {
                    full_number.push(self.current_char);
                    self.advance()
                }
                token_list.push(StrTokType::Integer(full_number.parse::<usize>().unwrap()))
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
                let mut is_dynamic = false;
                let mut size = 0;
                match self.current_token {
                    StrTokType::RightBracket => {
                        self.advance();
                        is_dynamic = true;
                    }
                    StrTokType::Integer(value) => {
                        self.advance();
                        size = value;
                    }
                    _ => {
                        eprintln!(
                            "[TypeParser] Expected a RightBracket, but recieved {:?}",
                            self.current_token
                        );
                        process::exit(1);
                    }
                }
                if is_dynamic {
                    Ok(Box::new(Type::DynamicArray(self.parse().unwrap())))
                } else {
                    Ok(Box::new(Type::FixedArray(size, self.parse().unwrap())))
                }
            }
            StrTokType::RightBracket => {
                eprintln!("[TypeParser] Found a random RightBracket");
                process::exit(1);
            }
            StrTokType::Identifier(id) => match id.as_str() {
                "int" => Ok(Box::new(Type::Primative("int".to_string()))),
                "char" => Ok(Box::new(Type::Primative("char".to_string()))),
                "usize" => Ok(Box::new(Type::Primative("usize".to_string()))),
                _ => {
                    eprintln!("[TypeParser] Resolving complex types does not exist yet!");
                    process::exit(1);
                }
            },
            _ => {
                eprintln!("[TypeParser] No way to parse `{:?}`", self.current_token);
                process::exit(1);
            }
        }
    }
}
