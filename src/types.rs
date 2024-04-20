use crate::{
    parser::{Operation, AST},
    scope::ScopeContext,
};

#[derive(PartialEq, Clone)]
pub enum Type {
    Primative(&'static str),
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

pub fn string_to_type(type_str: String) -> Result<Box<Type>, &'static str> {
    enum StrTokType {
        AtSign,
        Identifier(String),
    }
    let token_list = Vec::new();
    let i = 0;
    let current_char = type_str.bytes().nth(0).unwrap().try_into().unwrap();
    let get_char =
        |index: usize| -> char { type_str.bytes().nth(0).unwrap_or(0).try_into().unwrap() };
    let peek = |offset: i64| -> char { get_char(i.try_into().unwrap() + offset) };
    let advance = || {
        current_char = get_char(i + 1);
    };
    while i < type_str.len() {
        if current_char.is_alphabetic() {
        } else {
            token_list.push(match current_char {
                '@' => StrTokType::AtSign,
            });
            advance();
        }
    }
    Type::Star
}

pub fn calculate_expression_type(
    ast: Box<AST>,
    scope: &ScopeContext,
) -> Result<(Box<Type>, usize), &'static str> {
    //     // [TYPE, SIZE]
    //     match *ast {
    //         AST::Integer(_) => Ok(("int".to_string(), 8)),
    //         AST::UnaryExpr { child, .. } => Ok(calculate_expression_type(child, scope).unwrap()),
    //         AST::BinaryExpression { lhs, .. } => Ok(calculate_expression_type(lhs, scope).unwrap()),
    //         AST::VariableCall { name } => {
    //             let variable_type = scope.get_variable_data(name).0;
    //             Ok((variable_type.clone(), get_type_size(variable_type).unwrap()))
    //         }
    //         AST::FunctionCall { name, .. } => {
    //             let function_type = scope.get_function_data(name).0;
    //             Ok((function_type.clone(), get_type_size(function_type).unwrap()))
    //         }
    //         _ => Err("Unable to parse type of ast"),
    //     }

    let tree = ast_to_type_tree(ast, scope)?;
    let collapsed = collapse_type_tree(tree)?;
    Ok((collapsed.clone(), get_type_size(collapsed).unwrap()))
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

pub fn get_type_size(_comp: Box<Type>) -> Result<usize, &'static str> {
    Ok(8)
}

pub fn ast_to_type_tree(ast: Box<AST>, scope: &ScopeContext) -> Result<Box<Type>, &'static str> {
    match *ast {
        AST::Integer(_) => Ok(Box::new(Type::Primative("int"))),
        AST::String(_) => Ok(Box::new(Type::Primative("char *"))),
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
            Type::from_string(type_str)
        }
        _ => {
            eprintln!("{:?}", ast);
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
