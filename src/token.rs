use std::fmt;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: i32,
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<&str>, line: i32) -> Token {
        let lexeme = String::from(lexeme);
        let literal = match literal {
            Some(literal) => Some(String::from(literal)),
            None => None,
        };

        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal.as_ref().unwrap_or(&String::from("None")));
        fmt::Result::Ok(())
    }
}


// class Token {
// final TokenType type;
// final String lexeme;
// final Object literal;
// final int line;
//
// Token(TokenType type, String lexeme, Object literal, int line) {
// this.type = type;
// this.lexeme = lexeme;
// this.literal = literal;
// this.line = line;
// }
//
// public String toString() {
// return type + " " + lexeme + " " + literal;
// }
// }