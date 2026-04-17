#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn, Let, Struct, Model, Spawn, Chan, Return, Grad, Tensor,

    // Identifiers and Literals
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),

    // Operators
    Plus, Minus, Star, Slash, Assign, Arrow, Pipe,
    DoubleColon, Dot,

    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Colon, Comma, Semi,

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.pos];

        if ch.is_alphabetic() || ch == '_' {
            return self.read_identifier();
        }

        if ch.is_numeric() {
            return self.read_number();
        }

        self.pos += 1;
        match ch {
            '+' => Token::Plus,
            '-' => {
                if self.peek() == '>' {
                    self.pos += 1;
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => Token::Assign,
            '|' => Token::Pipe,
            '.' => Token::Dot,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => {
                if self.peek() == ':' {
                    self.pos += 1;
                    Token::DoubleColon
                } else {
                    Token::Colon
                }
            }
            ',' => Token::Comma,
            ';' => Token::Semi,
            _ => Token::EOF, // Should handle error
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        match s.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "struct" => Token::Struct,
            "model" => Token::Model,
            "spawn" => Token::Spawn,
            "chan" => Token::Chan,
            "return" => Token::Return,
            "grad" => Token::Grad,
            "tensor" => Token::Tensor,
            _ => Token::Ident(s),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let mut is_float = false;
        while self.pos < self.input.len() && (self.input[self.pos].is_numeric() || self.input[self.pos] == '.') {
            if self.input[self.pos] == '.' { is_float = true; }
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        if is_float {
            Token::Float(s.parse().unwrap_or(0.0))
        } else {
            Token::Int(s.parse().unwrap_or(0))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> char {
        if self.pos >= self.input.len() { '\0' } else { self.input[self.pos] }
    }
}
mod tests;
