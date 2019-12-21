
use std::{
    str::{ FromStr },
    // string::{ ToString },
    fmt::{ Debug, Display, Formatter, Result as fmt_Result }
};


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    USub,
} impl Operator {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
            'u' => Some(Self::USub),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::Pow => '^',
            Self::USub => 'u',
        }
    }

    pub fn evaluate(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
            Self::Pow => left.powf(right),
            Self::USub => -right,
        }
    }
} impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operator, Self::Err> {
        match Operator::from_char(s
                                .chars()
                                .next()
                                .expect("Cannot parse empty string into operator")
                            ) {
            Some(n) => Ok(n),
            None => Err("Unknown operator"),
        }
    }
} impl ToString for Operator {
    fn to_string(&self) -> String {
        format!("{}", self.to_char())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Paren {
    Left,
    Right,
} impl Paren {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Left),
            ')' => Some(Self::Right),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Left => '(',
            Self::Right => ')',
        }
    }
} impl FromStr for Paren {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Paren, Self::Err> {
        match Paren::from_char(s
                                .chars()
                                .next()
                                .expect("Cannot parse empty string into paren")
                            ) {
            Some(n) => Ok(n),
            None => Err("Unknown literal"),
        }
    }
} impl ToString for Paren {
    fn to_string(&self) -> String {
        format!("{}", self.to_char())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    Operator(Operator),
    Value(f64),
    Paren(Paren),
} impl Token {
    pub fn new(literal: &str) -> Self {
        match literal.parse::<Token>() {
            Ok(t) => t,
            Err(e) => panic!(e)
        }
    }

} impl FromStr for Token {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Token, Self::Err> {
        if let Ok(v) = s.parse::<f64>() {
            Ok(Token::Value(v))
        } else if let Ok(op) = s.parse::<Operator>() {
            Ok(Token::Operator(op))
        } else if let Ok(p) = s.parse::<Paren>() {
            Ok(Token::Paren(p))
        } else {
            Err("Unexpected literal")
        }
    } 


} impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", {
            match self {
                Token::Operator(op) => op.to_string(),
                Token::Paren(p) => p.to_string(),
                Token::Value(v) => v.to_string(),
            }
        })
    }
} impl Into<f64> for Token {
    fn into(self) -> f64 {
        match self {
            Token::Value(v) => v,
            _ => panic!("Attempt to coerce non-value Token to f64"),
        }
    }
}

/// Parse the string `s` into a Token stream
/// ```rust
/// let tokens = vec![
///     Token::new("("),
///     Token::new("10"),
///     Token::new("+"),
///     Token::new("5"),
///     Token::new(")"),
/// ];
/// assert!(tokens == tokenize("(10+5)"));
/// ```
pub fn tokenize(s: &str) -> Vec<Token> {
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    let cleaned = s.chars()
                .filter(|&c| ".0123456789/*-+^()".contains(c))
                .collect::<String>();

    let mut idx = 0;

    while let Some(c) = cleaned.chars().nth(idx) {
        // /*DEBUG:*/ eprint!("C: {}, IDX: {} -> ", c, idx);
        
        // check for unary operators (will always be first or directly following another operator (thanks greg!))
        // unwrap or will make this evalute true if it's the first item in the expression
        match tokens.last().unwrap_or(&Token::Operator(Operator::Add)) {
            Token::Operator(_) | Token::Paren(Paren::Left) => {
                if buffer.is_empty() && c == '-' {
                    if buffer.is_empty() {
                        // /*DEBUG:*/ eprintln!("Unary minus");
                        tokens.push(Token::Operator(Operator::USub));
                        idx += 1;
                        continue;
                    }
                }
            },
            _ => ()
        }

        // c is a number (0-9 or .), push it to the buffer
        if c.is_numeric() || c == '.' {
            // /*DEBUG:*/ eprintln!("Number: {}", c);
            buffer.push(c);
        }
        // if c is not a number, but there is something in the buffer, push the buffer to output
        else if !buffer.is_empty() {
            // /*DEBUG:*/ eprintln!("Commit number: {}", buffer);
            tokens.push(buffer.parse().expect(&format!("Failed to parse buffer: {:?}", buffer)));
            buffer = String::new();
            idx -= 1;
        }
        // Handle operators and parens normally
        else if let Some(op) = Operator::from_char(c) {
            // /*DEBUG:*/ eprintln!("Operator: {:?}", op);
            tokens.push(Token::Operator(op));
        } else if let Some(p) = Paren::from_char(c) {
            // /*DEBUG:*/ eprintln!("Paren: {:?}", p);
            tokens.push(Token::Paren(p));
        }

        idx += 1;
    }
    
    if !buffer.is_empty() {
        tokens.push(buffer
            .parse()
            .expect("Failed to parse token from buffer")
        );
    }

    tokens
}

fn precedence(token: &Token) -> u32 {
    match token {
        Token::Operator(o) => {
            match o {
                Operator::Add => 2,
                Operator::Sub => 2,
                Operator::Mul => 3,
                Operator::Div => 3,
                Operator::Pow => 4,
                Operator::USub => 5,
            }
        },
        _ => 0,
    }
}

#[derive(Copy, Clone, PartialEq)]
enum OperatorAssociativity {
    Left, Right
} impl From<Token> for OperatorAssociativity {
    fn from(token: Token) -> Self {
        match token {
            Token::Operator(Operator::Pow) | Token::Operator(Operator::USub) => OperatorAssociativity::Right,
            _ => OperatorAssociativity::Left,
        }
    }
} impl<T> From<&T> for OperatorAssociativity 
where 
    T: Clone,
    OperatorAssociativity: From<T>
{
    fn from(v: &T) -> Self {
        Self::from(v.clone())
    }
}


/// Takes an infix notated token stream and converts it to postfix notation
pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut opstack: Vec<Token> = Vec::new();
    
    for token in tokens {
        match token {
            Token::Value(_) => {
                output.push(token);
            },
            Token::Operator(_) => {
                let p = precedence(&token);
                while opstack.len() != 0 {
                    match opstack.last() {
                        Some(&Token::Paren(_)) => break,
                        Some(o) => {
                            if match OperatorAssociativity::from(&token) {
                                OperatorAssociativity::Left => {
                                    precedence(o) < p
                                },
                                OperatorAssociativity::Right => {
                                    precedence(o) <= p
                                },
                            } {
                                break
                            } else {
                                output.push(opstack.pop().unwrap());
                            }
                        },
                        _ => unreachable!()
                    }
                }
                opstack.push(token.clone());
            },
            Token::Paren(p) => {
                match p {
                    Paren::Left => {
                        opstack.push(token.clone())
                    },
                    Paren::Right => {
                        while opstack.len() != 0 {
                            if let Some(top) = opstack.pop() {
                                match top {
                                    Token::Paren(Paren::Left) => break,
                                    o => output.push(o),
                                }
                            } else {
                                unreachable!()
                            }
                        }
                    },
                }
            },
        }
    }

    while let Some(top) = opstack.pop() {
        output.push(top);
    }

    output
}

#[test]
fn test_tokenize() {
    // Basic
    let tokens = vec![
        Token::new("("),
        Token::new("10"),
        Token::new("+"),
        Token::new("5"),
        Token::new(")"),
    ];
    assert!(tokens == tokenize("(10+5)"));

    // Complex
    let tokens = vec![
        Token::new("("),
        Token::new("("),
        Token::new("10.0"),
        Token::new("*"),
        Token::new("2"),
        Token::new(")"),
        Token::new("/"),
        Token::new("4"),
        Token::new("+"),
        Token::new("("),
        Token::new("2.5"),
        Token::new("*"),
        Token::new("4"),
        Token::new(")"),
        Token::new("*"),
        Token::new("2"),
        Token::new(")"),
    ];
    assert!(tokens == tokenize("((10.0 * 2) / 4 + (2.5 * 4) * 2)"));

    // No parens
    let tokens = vec![
        Token::new("10"),
        Token::new("+"),
        Token::new("5"),
    ];
    assert!(tokens == tokenize("10.0 + 5"));

    // Unary minus
    let tokens = vec![
        Token::new("u"),
        Token::new("10"),
        Token::new("+"),
        Token::new("u"),
        Token::new("5"),
    ];
    assert!(tokens == tokenize("-10 + -5"));
    
}   

#[test]
fn test_shunting_yard() {
    let tokens = tokenize("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3");
    let expected = vec![
        Token::new("3"),
        Token::new("4"),
        Token::new("2"),
        Token::new("*"),
        Token::new("1"),
        Token::new("5"),
        Token::new("-"),
        Token::new("2"),
        Token::new("3"),
        Token::new("^"),
        Token::new("^"),
        Token::new("/"),
        Token::new("+"),
    ];

    assert_eq!(shunting_yard(tokens), expected);

    let tokens = tokenize("((15 / (7 -(1 + 1))) * 3) - (2 + (1 + 1))");
    let expected = vec![
        Token::new("15"),
        Token::new("7"),
        Token::new("1"),
        Token::new("1"),
        Token::new("+"),
        Token::new("-"),
        Token::new("/"),
        Token::new("3"),
        Token::new("*"),
        Token::new("2"),
        Token::new("1"),
        Token::new("1"),
        Token::new("+"),
        Token::new("+"),
        Token::new("-"),
    ];

    assert_eq!(shunting_yard(tokens), expected);
}