use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner{
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner{
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        for token in &self.tokens{
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", None, self.line));
        return &self.tokens;
    }

    fn scan_token(&self){

    }

    // private void scanToken() {
    // char c = advance();
    // switch (c) {
    // case '(': addToken(LEFT_PAREN); break;
    // case ')': addToken(RIGHT_PAREN); break;
    // case '{': addToken(LEFT_BRACE); break;
    // case '}': addToken(RIGHT_BRACE); break;
    // case ',': addToken(COMMA); break;
    // case '.': addToken(DOT); break;
    // case '-': addToken(MINUS); break;
    // case '+': addToken(PLUS); break;
    // case ';': addToken(SEMICOLON); break;
    // case '*': addToken(STAR); break;
    // }
    // }

    fn is_at_end(&self) -> bool {
        return self.current >= (self.source.len() as i32);
    }
}


// List<Token> scanTokens() {
// while (!isAtEnd()) {
// // We are at the beginning of the next lexeme.
// start = current;
// scanToken();
// }
//
// tokens.add(new Token(EOF, "", null, line));
// return tokens;
// }