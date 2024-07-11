use crate::parser::token::Token;

pub fn debug_token(expected: &str, found: &Token) -> String {
    format!("Expected {}, found {:?}", expected, found)
}

pub fn debug_log(message: &str, token: Option<&Token>) {
    if let Some(t) = token {
        println!("{}: {:?}", message, t);
    } else {
        println!("{}: No more tokens", message);
    }
}
