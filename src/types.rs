use std::process;

use crate::{
    parser::{Operation, AST},
    scope::ScopeContext, util::type_is_struct,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Primative(String),
    Slice(Box<Type>),
    FixedArray(usize, Box<Type>),
    Pointer(Box<Type>),
    UnaryOperation(Operation, Box<Type>),
    BinaryOperation(Operation, Box<Type>, Box<Type>),
    // Struct(String, Vec<Box<Type>>),
    Struct(String),
    Enum(String),
}

impl Type {
    pub fn to_string(&self) -> String {
        match self.clone() {
            Self::Primative(id) => id,
            Self::FixedArray(size, sub) => format!("[{}]{}", size, sub.to_string()),
            Self::Slice(sub) => format!("[]{}", sub.to_string()),
            Self::Pointer(sub) => format!("@{}", sub.to_string()),
            Self::Struct(name, ..) => name,
            _ => {
                eprintln!("[TypeParser] Cannot stringify type `{:?}`", self);
                process::exit(1);
            }
        }
    }
}

pub fn calculate_ast_type(ast: Box<AST>, scope: &ScopeContext) -> Result<Box<Type>, String> {
    let tree = ast_to_type_tree(ast, scope)?;
    let collapsed = collapse_type_tree(tree)?;
    Ok(collapsed.clone())
}

pub fn is_primative_type(potential: String) -> bool {
    ["int", "char", "usize", "void"].contains(&potential.as_str())
}

pub fn get_primative_type_size(prim: String) -> Result<usize, &'static str> {
    match prim.as_str() {
        "int" => Ok(8),
        "char" => Ok(1),
        "usize" => Ok(8),
        "void" => Ok(0),
        _ => Err("Not a primative type"),
    }
}

pub fn get_type_size(scope: &ScopeContext, comp: Box<Type>) -> Result<usize, &'static str> {
    match *comp {
        Type::Primative(prim) => get_primative_type_size(prim),
        Type::Pointer(_) => get_primative_type_size("usize".to_owned()),
        Type::UnaryOperation(_, sub) => get_type_size(scope, sub),
        Type::BinaryOperation(_, lhs, _) => get_type_size(scope, lhs),
        Type::FixedArray(size, sub) => Ok(get_type_size(scope, sub).unwrap() * size),
        Type::Slice(_) => get_primative_type_size("usize".to_owned()), // adding the size of the array onto the end of the space in memory, and since the size is a usize, then we add 8 bytes
        Type::Struct(name, ..) => {
            let members: Vec<Box<Type>> = scope.get_struct_data(name).iter().map(|x| string_to_collapsed_type_tree(x.1.clone(), scope).unwrap()).collect();
            let mut size = 0usize;
            for member in members {
                size += get_type_size(scope, member).unwrap();
            }
            Ok(size)
        }
        Type::Enum(_) => get_primative_type_size("usize".to_owned()),
    }
}

pub fn ast_to_type_tree(ast: Box<AST>, scope: &ScopeContext) -> Result<Box<Type>, String> {
    match *ast {
        AST::Integer(_) => Ok(Box::new(Type::Primative("int".to_string()))),
        AST::Character(_) => Ok(Box::new(Type::Primative("char".to_string()))),
        AST::String(_) => string_to_collapsed_type_tree(format!("[]char"), scope),
        AST::UnaryExpression { op, child } => {
            let child_type = ast_to_type_tree(child, scope)?;
            match op {
                Operation::Ref => Ok(Box::new(Type::Pointer(child_type))),
                Operation::Deref => match *child_type {
                    Type::Pointer(sub) => Ok(sub),
                    _ => {
                        eprintln!("[TypeParser] Non pointers cannot be de-referenced");
                        process::exit(1);
                    }
                },
                _ => Ok(Box::new(Type::UnaryOperation(op, child_type))),
            }
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            let lhs_type = ast_to_type_tree(lhs, scope)?;
            let rhs_type = ast_to_type_tree(rhs, scope)?;
            Ok(Box::new(Type::BinaryOperation(op, lhs_type, rhs_type)))
        }
        AST::VariableCall { name } => {
            let type_str = scope.get_variable_data(name).0;
            // println!("BUH: {:?}", type_str);
            let variable_type_tree = string_to_type_tree(type_str, scope).unwrap();
            Ok(variable_type_tree)
        }
        AST::FunctionCall { name, .. } => {
            if name == "sizeof" {
                return string_to_collapsed_type_tree("usize".to_owned(), scope);
            }
            let type_str = scope.get_function_data(name).0;
            let function_type_tree = string_to_type_tree(type_str, scope).unwrap();
            Ok(function_type_tree)
        }
        AST::Argument(sub) => ast_to_type_tree(sub, scope),
        AST::Struct { members, .. } => {
            let _member_types = members.iter().map(|x| x).for_each(|x| println!("{:?}", x));
            eprintln!("FAILURE!");
            process::exit(1);
            // Ok(Box::new(Type::Struct(members)))
        }
        AST::StructInitializer { name, .. } => Ok(string_to_collapsed_type_tree(name, scope)?),
        AST::MemberAccess { accessed, member } => {
            // println!("`{}` from `{:?}`", member, accessed);
            let accessed_type = collapse_type_tree(ast_to_type_tree(accessed.clone(), scope)?)?;
            let struct_name = match *accessed_type {
                Type::Struct(name, ..) => name,
                _ => {
                    eprintln!("Cannot access member of non-struct type");
                    process::exit(1);
                }
            };
            let struct_data = scope.get_struct_data(struct_name);
            let member_type_string = struct_data
                .iter()
                .find(|x| x.0 == member)
                .unwrap()
                .1
                .clone();
            string_to_collapsed_type_tree(member_type_string, scope)
        }
        AST::TypeCast { into, .. } => string_to_type_tree(into, scope),
        _ => {
            eprintln!("[TypeParser] {:?}", ast);
            Err("AST is not type-able".to_owned())
        }
    }
}

pub fn collapse_type_tree(tree: Box<Type>) -> Result<Box<Type>, String> {
    match *tree {
        Type::Primative(type_name) => Ok(Box::new(Type::Primative(type_name))),
        Type::UnaryOperation(op, child) => match op {
            Operation::Ref => Ok(Box::new(Type::Pointer(collapse_type_tree(child)?))),
            _ => collapse_type_tree(child),
        },
        Type::BinaryOperation(op, lhs, rhs) => {
            let collapsed_lhs = collapse_type_tree(lhs.clone())?;
            let collapsed_rhs = collapse_type_tree(rhs)?;

            if op == Operation::ArrAcc {
                return match *lhs {
                    Type::FixedArray(_, sub) => Ok(sub),
                    Type::Slice(sub) => Ok(sub),
                    _ => Err("Tried to do array access on non array type".to_owned()),
                };
            } else if collapsed_lhs != collapsed_rhs {
                return Err("Types do not match".to_owned());
            }
            Ok(collapsed_lhs)
        }
        Type::Pointer(sub_type) => Ok(Box::new(Type::Pointer(collapse_type_tree(sub_type)?))),
        Type::Slice(sub_type) => {
            Ok(Box::new(Type::Slice(collapse_type_tree(sub_type)?)))
        }
        Type::FixedArray(size, sub_type) => Ok(Box::new(Type::FixedArray(
            size,
            collapse_type_tree(sub_type)?,
        ))),
        Type::Struct(name, ..) => {
            Ok(Box::new(Type::Struct(name)))
            // let mut collapsed = Vec::new();
            // for member in members {
            //     collapsed.push(collapse_type_tree(member).unwrap());
            // }
            // Ok(Box::new(Type::Struct(name, collapsed)))
        }
        Type::Enum(name) => Ok(Box::new(Type::Enum(name))),
    }
}

pub fn string_to_type_tree(
    type_str: String,
    scope: &ScopeContext,
) -> Result<Box<Type>, String> {
    let type_tokens = TypeLexer::new(type_str).tokenize()?;
    TypeParser::new(type_tokens).parse(scope)
}

pub fn string_to_collapsed_type_tree(
    type_str: String,
    scope: &ScopeContext,
) -> Result<Box<Type>, String> {
    collapse_type_tree(string_to_type_tree(type_str, scope)?)
}

#[derive(Debug, Clone, PartialEq)]
enum StrTokType {
    AtSign,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Identifier(String),
    Integer(usize),
    EOS, // end of stream
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
            current_char: type_string.as_bytes().first().copied().unwrap_or(0).into(),
        }
    }
    fn get_char(&self, index: usize) -> char {
        self.type_string
            .as_bytes()
            .get(index)
            .copied()
            .unwrap_or(0)
            .into()
    }
    fn peek(&self, offset: i64) -> char {
        self.get_char((self.index as i64 + offset).try_into().unwrap())
    }
    fn advance(&mut self) {
        self.current_char = self.peek(1);
        self.index += 1;
    }
    pub fn tokenize(&mut self) -> Result<Vec<StrTokType>, String> {
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
                    '{' => StrTokType::LeftBrace,
                    '}' => StrTokType::RightBrace,
                    ',' => StrTokType::Comma,
                    _ => {
                        return Err(format!("[TypeParser] Character `{}` is not parseable in a type", self.current_char));
                    }
                });
                self.advance();
            }
        }
        Ok(token_list)
    }
}

#[derive(Debug)]
struct TypeParser {
    pub tokens: Vec<StrTokType>,
    pub index: usize,
    pub current_token: StrTokType,
}

impl TypeParser {
    pub fn new(tokens: Vec<StrTokType>) -> TypeParser {
        if tokens.len() > 0 {
            TypeParser {
                tokens: tokens.clone(),
                index: 0,
                current_token: tokens[0].clone(),
            }
        } else {
            eprintln!("No type to parse");
            process::exit(1);
        }
    }
    fn peek(&self, offset: i64) -> StrTokType {
        let new_index: usize = (self.index as i64 + offset)
            .try_into()
            .unwrap_or(self.tokens.len());
        if new_index < self.tokens.len() {
            return self.tokens[new_index].clone();
        }
        StrTokType::EOS
    }
    fn advance(&mut self) {
        self.current_token = self.peek(1);
        self.index += 1;
    }
    pub fn parse(&mut self, scope: &ScopeContext) -> Result<Box<Type>, String> {
        match self.current_token.clone() {
            StrTokType::AtSign => {
                self.advance();
                let pointer_type = self.parse(scope).unwrap();
                let pointer = Type::Pointer(pointer_type);
                Ok(Box::new(pointer))
            }
            StrTokType::LeftBracket => {
                self.advance();
                let mut is_dynamic = false;
                let mut length = 0;
                match self.current_token {
                    StrTokType::RightBracket => {
                        self.advance();
                        is_dynamic = true;
                    }
                    StrTokType::Integer(value) => {
                        self.advance();
                        self.advance();
                        length = value;
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
                    Ok(Box::new(Type::Slice(self.parse(scope).unwrap())))
                } else {
                    Ok(Box::new(Type::FixedArray(length, self.parse(scope).unwrap())))
                }
            }
            StrTokType::RightBracket => {
                eprintln!("[TypeParser] Found a random RightBracket");
                process::exit(1);
            }
            StrTokType::Identifier(id) => match id.as_str() {
                "int" => Ok(Box::new(Type::Primative("int".to_owned()))),
                "char" => Ok(Box::new(Type::Primative("char".to_owned()))),
                "usize" => Ok(Box::new(Type::Primative("usize".to_owned()))),
                "void" => Ok(Box::new(Type::Primative("void".to_owned()))),
                _ => {
                    if type_is_struct(scope, id.clone()) {
                        Ok(Box::new(Type::Struct(id)))
                    } else if let Some((_, alias)) = scope.defined_types.iter().find(|x| x.0 == id) {
                        Ok(string_to_type_tree(alias.to_owned(), scope)?)
                    } else {
                        Err(format!("Could not find type {id}"))
                    }
                }
            }
            StrTokType::LeftBrace => {
                self.advance();
                let mut members = Vec::new();
                while self.current_token != StrTokType::RightBrace {
                    let mut buf_str = String::new();
                    while self.current_token != StrTokType::Comma
                        && self.current_token != StrTokType::RightBrace
                        && self.current_token != StrTokType::Comma
                    {
                        buf_str.push_str(
                            match self.current_token.clone() {
                                StrTokType::AtSign => "@".to_string(),
                                StrTokType::Integer(val) => val.to_string(),
                                StrTokType::LeftBracket => "[".to_string(),
                                StrTokType::RightBracket => "]".to_string(),
                                StrTokType::LeftBrace => "{".to_string(),
                                StrTokType::RightBrace => "{".to_string(),
                                StrTokType::Comma => ",".to_string(),
                                StrTokType::Identifier(id) => id,
                                StrTokType::EOS => panic!("Reached end of stream\n{buf_str}"),
                            }
                            .as_str(),
                        );
                        self.advance();
                    }
                    members.push(string_to_collapsed_type_tree(buf_str, scope).unwrap());
                    if self.current_token == StrTokType::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                Ok(Box::new(Type::Struct("".to_owned())))
            }
            _ => {
                eprintln!("[TypeParser] No way to parse `{:?}`", self.current_token);
                process::exit(1);
            }
        }
    }
}
