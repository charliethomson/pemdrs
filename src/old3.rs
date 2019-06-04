/*
IDEA:
    `token`: any item in an expression; can be a number, function, or operator
    `expression`: wrapper for Vec<Token>; works on tokens in bulk

*/

use std::{
    alloc::{ alloc_zeroed, dealloc, realloc, Layout },
};


fn is_valid_operator(c: &char) -> bool {
    ['(', ')', '+', '-', '*', '/', '^'].contains(c)
}

#[derive(Clone)]
struct Token {
    value: String,
} impl Token {
    fn new() -> Token {
        Token {
            value: String::new()
        }
    }
    
    fn from(s: String) -> Token {
        Token {
            value: s,
        }
    }

    fn has_value(&self) -> bool {
        !self.value.is_empty()
    }

    fn as_string(&self) -> String {
        self.value.clone()
    }

    fn as_char(&self) -> char {
        self.value.chars().nth(0).unwrap()
    }

    fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    fn is_numeric(&self) -> bool {
        for c in self.value.chars() {
            if !c.is_numeric() { return false; }
        }
        true
    }

    fn is_operator(&self) -> bool {
        is_valid_operator(&self.as_char())
    }

    fn is_paren(&self) -> bool {
        self.as_char() == '(' || self.as_char() == ')'
    }
}

struct Expression {
    tokens: Vec<Token>,
} impl Expression {
    fn new() -> Expression {
        Expression { tokens: Vec::<Token>::new() }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn from_string(s: &String) -> Expression {

        let ops = vec!['(', ')', '+', '-', '*', '/', '^'];
        let mut exp = Expression::new();

        let mut buf = String::new();
        let mut chariter = s.chars();
        while let Some(c) = chariter.next() {
            if c.is_whitespace() {
                continue;
            } else if c.is_numeric() {
                buf.push(c);
                if let Some(ch) = chariter.next() {
                    if ch.is_numeric() {
                        buf.push(ch);
                    } else {
                        if ch.is_whitespace() { continue; }
                        exp.add_token(Token::from(buf.clone()));
                        let mut buf = String::new();
                        buf.push(ch);
                        exp.add_token(Token::from(buf.clone()));
                    }
                } else {
                    exp.add_token(Token::from(buf.clone()));
                    buf = String::new();
                }
            } else if ops.contains(&c) {
                if buf.len() != 0 {
                    exp.add_token(Token::from(buf.clone()));
                    buf = String::new();
                }
                buf.push(c);
                exp.add_token(Token::from(buf.clone()));
                buf = String::new();
            }
            

        };
        return exp;
    }

    fn tokens(&mut self) -> Vec<Token> {
        self.tokens.clone()
    }
    
    fn as_string(&self) -> String {
        let output = String::new();
        for token in self.tokens() {
            output.extend(token.as_string().chars());
        };
        return output;
    }

    unsafe fn eval<T>(&mut self) -> T {
        let output: ;


        return *output;
    }
}

fn eval_expression(expr: &mut Expression) {
    
    for token in expr.tokens().iter() {
        eprintln!("Token: {}", token.value);
    }
}

fn main() {
    let mut e = Expression::from_string(&String::from("10+((11 / 2)^2)"));
    eprintln!("{} = {}", e.as_string(), e.eval::<i32>());
}