use crate::parser::AST;

pub struct PreProcessor {
    pub typedef: Vec<()>, // [TYPE, VALUE]
    pub definitions: Vec<(String, Box<AST>)>, // [NAME, VALUE]
}

impl PreProcessor {
    pub fn new() -> PreProcessor {
        PreProcessor {
            typedef: Vec::new(),
            definitions: Vec::new(),
        }
    }
    pub fn preprocess(&mut self, ast: Box<AST>) -> (Box<AST>, bool) { // [AST, MODIFIED?]
        /* TODO: Implement preprocessing (duh)
            1. keep iterating over the ast until no changes have been made
            2. return final ast
        */
        let result = match *ast.clone() {
            AST::Null => (Box::new(AST::Null), false),
            AST::Integer(value) => (Box::new(AST::Integer(value)), false),
            AST::String(value) => (Box::new(AST::String(value)), false),
            AST::Character(value) => (Box::new(AST::Character(value)), false),
            AST::UnaryExpression { op, child } => {
                let child = self.preprocess(child);
                (Box::new(AST::UnaryExpression {
                    op,
                    child: child.0,
                }), child.1)
            }
            AST::BinaryExpression { op, lhs, rhs } => {
                let lhs = self.preprocess(lhs);
                let rhs = self.preprocess(rhs);
                (Box::new(AST::BinaryExpression {
                    op,
                    lhs: lhs.0,
                    rhs: rhs.0,
                }), lhs.1 || rhs.1)
            }
            AST::Argument(value) => {
                let new_value = self.preprocess(value);
                (Box::new(AST::Argument(new_value.0)), new_value.1)
            },
            AST::Parameter { param_type, name } => (Box::new(AST::Parameter { param_type , name }), false),
            AST::FunctionDeclaration { name, return_type, prototype, body } => {
                let prototype: Vec<(Box<AST>, bool)> = prototype.iter().map(|x| self.preprocess(x.clone())).collect();
                let prototype_values = prototype.iter().map(|x| x.0.clone()).collect();
                let prototype_mod = prototype.iter().any(|x| x.1);
                let body = self.preprocess(body);
                (Box::new(AST::FunctionDeclaration {
                    name, 
                    return_type,
                    prototype: prototype_values,
                    body: body.0,
                }), prototype_mod || body.1)
            }
            AST::FunctionCall { name, arguments } => {
                let arguments: Vec<(Box<AST>, bool)> = arguments.iter().map(|x| self.preprocess(x.clone())).collect();
                let arguments_values = arguments.iter().map(|x| x.0.clone()).collect();
                let arguments_mod = arguments.iter().any(|x| x.1);
                (Box::new(AST::FunctionCall {
                    name,
                    arguments: arguments_values,
                }), arguments_mod)
            }
            AST::VariableDeclaration { variable_type, name } => (Box::new(AST::VariableDeclaration { variable_type, name }), false),
            AST::VariableAssignment { name, value } => {
                let value = self.preprocess(value);
                (Box::new(AST::VariableAssignment {
                    name,
                    value: value.0,
                }), value.1)
            }
            AST::VariableCall { name } => {
                // eprintln!("Looking for definition: {name}");
                if let Some(def) = self.definitions.iter().find(|x| x.0 == name) {
                    (def.1.clone(), true)
                } else {
                    (Box::new(AST::VariableCall { name }), false)
                }
            },
            AST::If { condition, body, alt } => {
                let condition = self.preprocess(condition);
                let body = self.preprocess(body);
                let alt = if let Some(val) = alt {
                    let new = self.preprocess(val);
                    (Some(new.0), new.1)
                } else {
                    (alt, false)
                };
                (Box::new(AST::If {
                    condition: condition.0,
                    body: body.0,
                    alt: alt.0,
                }), condition.1 || body.1 || alt.1)
            }
            AST::While { condition, body } => {
                let condition = self.preprocess(condition);
                let body = self.preprocess(body);
                (Box::new(AST::While {
                    condition: condition.0,
                    body: body.0
                }), condition.1 || body.1)
            }
            // AST::For { initializer, condition, updater, body } => {}
            AST::Return(value) => {
                let new_value = self.preprocess(value);
                (Box::new(AST::Return(new_value.0)), new_value.1)
            },
            AST::Asm(code) => (Box::new(AST::Asm(code)), false),
            AST::Block(statements) => {
                let statements: Vec<(Box<AST>, bool)> = statements.iter().map(|x| self.preprocess(x.clone())).collect();
                let statements_values = statements.iter().map(|x| x.0.clone()).collect();
                let statements_mod = statements.iter().any(|x| x.1);
                (Box::new(AST::Block(statements_values)), statements_mod)
            }
            AST::Import { module } => (Box::new(AST::Import { module }), false),
            AST::Struct { name, members } => (Box::new(AST::Struct { name, members }), false),
            AST::Enum { name, values, attributes } => {
                // TODO: Attributes
                let prefix = if attributes.iter().any(|x| x == "noprefix") { "".to_string() } else { format!("{name}::") };
                for (i, value) in values.iter().enumerate() {
                    self.definitions.push(
                        (format!("{prefix}{value}"), Box::new(AST::Integer(i as i64)))
                    );
                }
                (Box::new(AST::TypeDefinition { name, type_string: "int".to_string() }), true)
            }
            // AST::StructInitializer { spreads, name, members } => {}
            AST::MemberAccess { .. } => (ast, false),
            AST::TypeDefinition { .. } => (ast, false),
            AST::Definition { name, value } => {
                let reproc = self.preprocess(value);
                self.definitions.push((name, reproc.0));
                (Box::new(AST::Null), true)
            }
            _ => todo!("{:#?}", ast),
        };
        if result.1 {
            self.preprocess(result.0)
        } else {
            result
        }
    }
}
