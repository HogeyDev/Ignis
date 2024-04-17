use crate::{
    parser::{Operation, AST},
    scope::ScopeContext,
};

// pub fn calculate_expression_type(
//     ast: Box<AST>,
//     scope: &ScopeContext,
// ) -> Result<(String, usize), &'static str> {
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
// }

#[derive(PartialEq)]
pub enum Type {
    Primative(&'static str),
    Array(Box<Type>),
    Pointer(Box<Type>),
    UnaryOperation(Operation, Box<Type>),
    BinaryOperation(Operation, Box<Type>, Box<Type>),
}

// impl Type {
//     pub fn to_string(&self) -> String {
//         "totally a real type".to_string()
//     }
// }

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

pub fn get_type_size(ty: String) -> Result<usize, &'static str> {
    Ok(8)
}

pub fn ast_to_type_tree(ast: Box<AST>) -> Result<Box<Type>, &'static str> {
    match *ast {
        AST::Integer(_) => Ok(Box::new(Type::Primative("int"))),
        AST::String(_) => Ok(Box::new(Type::Primative("char *"))),
        AST::UnaryExpr { op, child } => {
            let child_type = ast_to_type_tree(child)?;
            Ok(Box::new(Type::UnaryOperation(op, child_type)))
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            let lhs_type = ast_to_type_tree(lhs)?;
            let rhs_type = ast_to_type_tree(rhs)?;
            Ok(Box::new(Type::BinaryOperation(op, lhs_type, rhs_type)))
        }
        _ => Err("AST is not type-able"),
    }
}

pub fn collapse_type_tree(tree: Box<Type>) -> Result<Box<Type>, &'static str> {
    match *tree {
        Type::Primative(type_name) => Ok(Box::new(Type::Primative(type_name))),
        Type::UnaryOperation(op, child) => collapse_type_tree(child),
        Type::BinaryOperation(op, lhs, rhs) => {
            let collapsed_lhs = collapse_type_tree(lhs)?;
            let collapsed_rhs = collapse_type_tree(rhs)?;

            if collapsed_lhs != collapsed_rhs {
                return Err("Types do not match");
            }
            Ok(collapsed_lhs)
        }
        Type::Pointer(sub_type) => Ok(Box::new(Type::Pointer(collapse_type_tree(sub_type)?))),
        Type::Array(sub_type) => Ok(Box::new(Type::Pointer(collapse_type_tree(sub_type)?))),
    }
}
