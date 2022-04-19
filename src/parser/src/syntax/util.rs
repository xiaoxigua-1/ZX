use util::ast::Expression;
use util::error::ZXError;
use util::token::Position;

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