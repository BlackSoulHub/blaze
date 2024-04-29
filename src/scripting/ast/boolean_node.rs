use std::io;

use crate::scripting::tokens::TokenType;

use super::expression_node::ExpressionNode;


pub struct BooleanNode {
    state: bool
}

impl BooleanNode {
    pub fn new (token_type: TokenType) -> Result<Self, io::Error> {
        match token_type {
            TokenType::True => {
                return Ok(BooleanNode { state: true })
            },
            TokenType::False => {
                return Ok(BooleanNode { state: false})
            },
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid value for a boolean node"))
        }
    }
}

impl ExpressionNode for BooleanNode {}

impl Default for BooleanNode {
    fn default () -> Self {
        Self::new(TokenType::False).unwrap()
    }
}
