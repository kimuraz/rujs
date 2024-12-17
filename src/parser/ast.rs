use crate::lexical::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    VariableDeclaration {
        identifier: String,
        value: Box<ASTNode>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
        else_body: Vec<ASTNode>,
    },
    Block(Vec<ASTNode>),
    ExpressionStatement(Box<ASTNode>),
    AssignmentExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    BinaryExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Literal(String),
    Identifier(String),
}

impl ASTNode {
    pub fn new_program() -> Self {
        ASTNode::Program(vec![])
    }
    pub fn add_to_program(&mut self, node: ASTNode) {
        if let ASTNode::Program(nodes) = self {
            nodes.push(node);
        }
    }
    pub fn new_variable_declaration(identifier: String, value: ASTNode) -> Self {
        ASTNode::VariableDeclaration {
            identifier,
            value: Box::new(value),
        }
    }
    pub fn new_function_declaration(name: String, parameters: Vec<String>, body: Vec<ASTNode>) -> Self {
        ASTNode::FunctionDeclaration {
            name,
            parameters,
            body,
        }
    }
    pub fn new_if_statement(condition: ASTNode, body: Vec<ASTNode>, else_body: Vec<ASTNode>) -> Self {
        ASTNode::IfStatement {
            condition: Box::new(condition),
            body,
            else_body,
        }
    }
    pub fn new_block(nodes: Vec<ASTNode>) -> Self {
        ASTNode::Block(nodes)
    }
    pub fn new_expression_statement(expression: ASTNode) -> Self {
        ASTNode::ExpressionStatement(Box::new(expression))
    }
    pub fn new_assignment_expression(left: ASTNode, operator: String, right: ASTNode) -> Self {
        ASTNode::AssignmentExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn new_binary_expression(left: ASTNode, operator: String, right: ASTNode) -> Self {
        ASTNode::BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn new_literal(value: String) -> Self {
        ASTNode::Literal(value)
    }
    pub fn new_identifier(name: String) -> Self {
        ASTNode::Identifier(name)
    }

    pub fn parse_program(tokens: &[Token]) -> ASTNode {
        let mut index = 0;
        let mut nodes = Vec::new();

        while index < tokens.len() {
            let statement = Self::parse_statement(tokens, &mut index);
            nodes.push(statement);

            if let Some(Token::EOF) = tokens.get(index) {
                break;
            }
        }

        ASTNode::Program(nodes)
    }

    pub fn parse_statement(tokens: &[Token], index: &mut usize) -> ASTNode {
        match tokens.get(*index) {
            Some(Token::Keyword(ref kw)) if kw == "let" => {
                *index += 1; // Consume 'let'
                let identifier = match tokens.get(*index) {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => panic!("Expected identifier after 'let'"),
                };
                *index += 1;

                match tokens.get(*index) {
                    Some(Token::Operator(op)) if op == "=" => *index += 1,
                    _ => panic!("Expected '=' after variable name"),
                }

                let value = Self::parse_expression(tokens, index);

                if let Some(Token::Delimiter(d)) = tokens.get(*index) {
                    if d == ";" {
                        *index += 1;
                    }
                }

                ASTNode::VariableDeclaration {
                    identifier,
                    value: Box::new(value),
                }
            }
            _ => panic!("Unsupported statement or token: {:?}", tokens.get(*index)),
        }
    }

    pub fn parse_expression(tokens: &[Token], index: &mut usize) -> ASTNode {
        match tokens.get(*index) {
            Some(Token::Literal(value)) => {
                *index += 1;
                ASTNode::Literal(value.clone())
            }
            Some(Token::Identifier(name)) => {
                *index += 1;
                ASTNode::Identifier(name.clone())
            }
            _ => panic!("Unsupported expression at index {}", index),
        }
    }

    pub fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        let padding = " ".repeat(indent * 2);

        match self {
            ASTNode::Program(nodes) => {
                result.push_str(&format!("{}Program:\n", padding));
                for node in nodes {
                    result.push_str(&node.pretty_print(indent + 1));
                }
            }
            ASTNode::VariableDeclaration { identifier, value } => {
                result.push_str(&format!(
                    "{}VariableDeclaration: {} = {}\n",
                    padding,
                    identifier,
                    value.pretty_print(indent + 1)
                ));
            }
            ASTNode::FunctionDeclaration { name, parameters, body } => {
                result.push_str(&format!(
                    "{}FunctionDeclaration: {}({:?})\n",
                    padding, name, parameters
                ));
                for node in body {
                    result.push_str(&node.pretty_print(indent + 1));
                }
            }
            ASTNode::IfStatement { condition, body, else_body } => {
                result.push_str(&format!("{}IfStatement:\n", padding));
                result.push_str(&format!("{}Condition:\n", " ".repeat((indent + 1) * 2)));
                result.push_str(&condition.pretty_print(indent + 2));
                result.push_str(&format!("{}Then:\n", " ".repeat((indent + 1) * 2)));
                for node in body {
                    result.push_str(&node.pretty_print(indent + 2));
                }
                if !else_body.is_empty() {
                    result.push_str(&format!("{}Else:\n", " ".repeat((indent + 1) * 2)));
                    for node in else_body {
                        result.push_str(&node.pretty_print(indent + 2));
                    }
                }
            }
            ASTNode::Block(nodes) => {
                result.push_str(&format!("{}Block:\n", padding));
                for node in nodes {
                    result.push_str(&node.pretty_print(indent + 1));
                }
            }
            ASTNode::ExpressionStatement(expression) => {
                result.push_str(&format!("{}ExpressionStatement:\n", padding));
                result.push_str(&expression.pretty_print(indent + 1));
            }
            ASTNode::AssignmentExpression { left, operator, right } => {
                result.push_str(&format!("{}AssignmentExpression: {}\n", padding, operator));
                result.push_str(&left.pretty_print(indent + 1));
                result.push_str(&right.pretty_print(indent + 1));
            }
            ASTNode::BinaryExpression { left, operator, right } => {
                result.push_str(&format!("{}BinaryExpression: {}\n", padding, operator));
                result.push_str(&left.pretty_print(indent + 1));
                result.push_str(&right.pretty_print(indent + 1));
            }
            ASTNode::Literal(value) => {
                result.push_str(&format!("{}Literal: {}\n", padding, value));
            }
            ASTNode::Identifier(name) => {
                result.push_str(&format!("{}Identifier: {}\n", padding, name));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::lexical::lexer::Token;
    use super::*;

    #[test]
    fn test_parse_program() {
        let tokens = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("=".to_string()),
            Token::Literal("42".to_string()),
            Token::Delimiter(";".to_string()),
        ];

        let ast = ASTNode::parse_program(&tokens);
        let expected_ast = ASTNode::Program(vec![
            ASTNode::VariableDeclaration {
                identifier: "x".to_string(),
                value: Box::new(ASTNode::Literal("42".to_string())),
            },
        ]);

        assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
        assert_eq!(ast.pretty_print(0), "bla");
    }
}
