
use crate::tree::{ Operator, Paren };

#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    Operator(Operator),
    Value(f64),
    Paren(Paren),
} impl Token {
    pub fn new(literal: &str) -> Self {
        if let Ok(v) = literal.parse::<f64>() {
            Token::Value(v)
        } else if let Some(op) = Operator::from_char(literal
                                                        .chars()
                                                        .next()
                                                        .expect("Cannot create node from empty literal")) {
            Token::Operator(op)
        } else {
            panic!("Unexpected literal: '{:?}', failed classification", literal);
        }
    }
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    

    tokens
}

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    Vec::new()
}