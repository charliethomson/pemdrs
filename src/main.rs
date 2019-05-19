
use std::{
    collections::{VecDeque, HashMap},
    iter::FromIterator,  
};

enum Paren {
    Open,
    Close,
} impl std::fmt::Debug for Paren {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Paren::Open => "Open",
            Paren::Close => "Close",
        })
    }
}

enum Assoc {
    Left,
    Right,
} impl std::fmt::Debug for Assoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Assoc::Left => "Left",
            Assoc::Right => "Right",
        })
    }
}



#[derive(Clone, Hash, PartialEq, Eq)]
struct Token {
    s: String,
} impl Token {
    fn new(s: &String) -> Token {
        Token { s: s.clone() }
    }

    fn from_char(c: char) -> Token {
        let mut s = String::new();
        s.push(c);
        Token { s }
    }

    fn unwrap(&self) -> String {
        self.s.clone()
    }

    fn as_char(&self) -> char {
        self.s.chars().nth(0).unwrap()
    }


    fn is_numeric(&self) -> bool {
        for c in self.s.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }

    fn associativity(&self) -> Option<Assoc> {
        if !self.is_operator() { return None; }
        let c = self.as_char();
        if c == '^' {
            return Some(Assoc::Right);
        } else {
            return Some(Assoc::Left);
        }
    }

    fn is_left_assoc(&self) -> bool {
        match self.associativity() {
            Some(Assoc::Left) => true,
            _ => false,
        }
    }


    fn precedence(&self) -> u8 {
        match self.as_char() {
            '+' | '-' => 2,
            '*' | '/' => 3,
            '^'       => 4,
            _         => 0,
        }
    }

    fn is_operator(&self) -> bool {
        if self.s.len() != 1 || self.is_numeric() { return false; }
        return vec!['(', ')', '+', '-', '*', '/', '^'].contains(&self.as_char());
        
    }

    fn is_paren(&self) -> Option<Paren> {
        if !self.is_operator() { return None; }
        let c = self.as_char();
        if c == ')' {
            return Some(Paren::Close);
        } else if c == '(' {
            return Some(Paren::Open);
        } else {
            return None;
        }
    }
} impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\ntoken: {:?} numeric? {}, operator? {}, paren? {:?}", self.s, self.is_numeric(), self.is_operator(), self.is_paren())
    }
} impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

struct Expression {
    tokens: Vec<Token>,
} impl Expression {
    fn new() -> Expression {
        Expression { tokens: Vec::new() }
    }
    
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn from_string(s: String) -> Expression {

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
                        exp.add_token(Token::new(&buf));
                        let mut buf = String::new();
                        buf.push(ch);
                        exp.add_token(Token::new(&buf));
                    }
                } else {
                    exp.add_token(Token::new(&buf));
                    buf = String::new();
                }
            } else if ops.contains(&c) {
                if buf.len() != 0 {
                    exp.add_token(Token::new(&buf));
                    buf = String::new();
                }
                buf.push(c);
                exp.add_token(Token::new(&buf));
                buf = String::new();
            }
            

        };
        return exp;
    }

    fn tokens(&mut self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn to_string(&self) -> String {
        let mut buf = String::new();

        for token in self.tokens.clone() {
            buf.push_str(&token.s.clone());
            buf.push(' ');
        }

        return buf;
    }
}

fn top<T>(v: &mut VecDeque<T>) -> Option<&T> {
    match v.len() {
        0 => None,
        n => Some(&v[n - 1])
    }
}

fn shunting_yard(expr: &mut Expression) -> Result<Expression, String> {
    /* `output queue` */
    let mut output = Expression::new();
    /* `operator stack` */
    let mut opstack = VecDeque::<Token>::new();
    'main: for token in expr.tokens() {
        if token.is_numeric() {
            output.add_token(token.clone());
        } else if token.is_operator() {
            // opstack.push_front(token.clone());
            /* if the token is a paren */
            if let Some(paren) = token.is_paren() {
                match paren {
                    /* opening */
                    Paren::Open  => {
                        opstack.push_front(token);
                    },
                    /* opening */
                    Paren::Close => {
                        'search: loop {
                            eprintln!("(search loop) opstack: len: {}; data: {:#?};", opstack.len(), opstack);
                            match top(&mut opstack) {
                                /* if there is a top of the stack */
                                Some(_) => {
                                    /* get the value and match it with open, close */
                                    if let Some(stacktop) = opstack.pop_back() {
                                        match stacktop.is_paren() {
                                            /* if it's an open, pop the operator, discard, and break the loop*/
                                            Some(Paren::Open)  => { break 'search; },
                                            /* if it's a close, do nothing (pop and discard) */
                                            Some(Paren::Close) => (),
                                            /* otherwise, pop the operator the to output stack */
                                            None               => { output.add_token(stacktop); },
                                        }
                                    }
                                }
                                None => {
                                    eprintln!("{}", output.to_string());
                                    return Err(String::from("Mismatched parens! error 1"));
                                }
                            }
                        }
                    },
                };

            /* if it's an operator but not a paren */
            } else {
                let mut check = false;
                'check: while let Some(stacktop) = top(&mut opstack) {
                    if { if let Some(paren) = stacktop.is_paren() { match paren { Paren::Open => true, _ => false} } else { false } } {
                        /* code omitted */
                    } else if stacktop.precedence() > token.precedence() {
                        /* code omitted */
                    } else if stacktop.precedence() == token.precedence() && stacktop.is_left_assoc() {
                        /* code omitted */
                    }

                    if check { break 'check; }
                    else { output.add_token(opstack.pop_back().unwrap()); }
                }
                /* if a and b and c in s -> if all( (a in s, b in s, c in s) ) */
                if !check {
                    // return Err
                }
                opstack.push_front(token);
            }
        } else { return Err(format!("Unexpected character {}", token.as_char())); }
    }

    while !opstack.is_empty() {
        if let Some(stacktop) = opstack.pop_back() {
            eprintln!("len: {}\nstack: {:#?}", opstack.len(),opstack);
            if let Some(_) = stacktop.is_paren() {
                return Err(String::from("Mismatched parens! error 3"));
            }
            output.add_token(stacktop.clone());
        }
    }


    return Ok(output);
}

fn main() {
    // let input = String::from("10 + (5 * 3) / (10 + 431 / (1131 ^ 2))");
    let input = String::from("3 + 4 * 2 / (1 - 5) ^ 2 ^ 3");
    let mut e = Expression::from_string(input.clone());
    for token in e.tokens.iter() {
        eprintln!("{:?}", token);
    }

    eprintln!("input: {}; expression: {}", input, e.to_string());

    eprintln!("{}", shunting_yard(&mut e).unwrap().to_string());
}