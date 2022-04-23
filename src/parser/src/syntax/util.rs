use util::ast::Operator;
use util::error::ZXError;
use util::token::{Position, Token, Tokens};

pub fn set_error_message<T>(expression: Result<T, ZXError>, message: String, pos: &Position) -> Result<T, ZXError> {
    match expression {
        Err(_) => {
            Err(ZXError::SyntaxError {
                message,
                pos: pos.clone(),
            })
        }
        Ok(condition) => Ok(condition)
    }
}

pub fn operator_type(token: &Token) -> Result<Operator, ZXError> {
    Ok(match token.token_type {
        Tokens::PlusToken => Operator::Add,
        Tokens::MultiplyToken => Operator::Mul,
        Tokens::MinusToken => Operator::Sub,
        _ => return Err(ZXError::SyntaxError {
            pos: token.pos.clone(),
            message: "not an operator".to_string(),
        }),
    })
}

pub fn is_operator(token_type: &Tokens) -> bool {
    match token_type {
        Tokens::PlusToken | Tokens::MultiplyToken | Tokens::MinusToken => true,
        _ => false
    }
}

pub fn infix_binding_power(operator_type: &Operator) -> u8 {
    match operator_type {
        Operator::Add | Operator::Sub => 1,
        Operator::Mul => 3,
    }
}