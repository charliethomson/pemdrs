
use std::{
    str::FromStr,
    fmt::{ Debug, Display, Formatter, Result as fmt_Result }
};


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow
} impl Operator {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
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
        }
    }

    pub fn evaluate(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
            Self::Pow => left.powf(right),
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
                Token::Operator(op) => format!("{}", op.to_char()),
                Token::Paren(p) => format!("{}", p.to_char()),
                Token::Value(v) => format!("{}", v)
            }
        })
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
        if c.is_numeric() || c == '.' {
            buffer.push(c);
        } else if !buffer.is_empty() {
            tokens.push(buffer
                    .parse()
                    .expect("Failed to parse token from buffer")
                );
            buffer = String::new();
            idx -= 1;
        } else if let Ok(token) = c.to_string().parse() {
            tokens.push(token);
        } else {
            panic!("Encountered unexpected character '{}'", c);
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

#[derive(Clone, Copy, PartialEq)]
enum OpWrapper {
    Operator(Operator),
    Paren(Paren),
} impl OpWrapper {
    fn to_token(self) -> Token {
        match self {
            OpWrapper::Operator(op) => Token::Operator(op),
            OpWrapper::Paren(p) => Token::Paren(p),
        }
    }
} impl From<Operator> for OpWrapper {
    fn from(op: Operator) -> Self {
        OpWrapper::Operator(op)
    }
} impl From<Paren> for OpWrapper {
    fn from(p: Paren) -> Self {
        OpWrapper::Paren(p)
    }
}


fn precedence(op: &OpWrapper) -> u32 {
    match op {
        OpWrapper::Operator(o) => {
            match o {
                Operator::Add => 2,
                Operator::Sub => 2,
                Operator::Mul => 3,
                Operator::Div => 3,
                Operator::Pow => 4,
            }
        }, 
        _ => 0
        
    }
}

#[derive(Copy, Clone, PartialEq)]
enum OperatorAssociativity {
    Left, Right
} impl From<OpWrapper> for OperatorAssociativity {
    fn from(op: OpWrapper) -> Self {
        match op {
            OpWrapper::Operator(Operator::Pow) => OperatorAssociativity::Right,
            _ => OperatorAssociativity::Left,
        }
    }
} impl<T> From<&T> for OperatorAssociativity where T: Clone, OperatorAssociativity: From<T> {
    fn from(v: &T) -> Self {
        Self::from(v.clone())
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut opstack: Vec<OpWrapper> = Vec::new();
    
    for token in tokens {
        match token {
            Token::Value(_) => {
                output.push(token);
            },
            Token::Operator(op) => {
                while 
                    !opstack.is_empty() 
                    &&
                    (
                        precedence(&OpWrapper::from(op)) < precedence(opstack.last().unwrap())
                        ||
                        (
                            precedence(&OpWrapper::from(op)) == precedence(opstack.last().unwrap()) 
                            && 
                            OperatorAssociativity::from(opstack.last().unwrap()) == OperatorAssociativity::Left
                        )
                        &&
                        (opstack.last().unwrap() != &OpWrapper::Paren(Paren::Left)
                    )
                ) {
                        output.push(opstack.pop().unwrap().to_token())
                }

                opstack.push(OpWrapper::from(op));
            },
            Token::Paren(p) => {
                match p {
                    Paren::Left => opstack.push(OpWrapper::from(p)),
                    Paren::Right => {
                        while opstack.last().expect("Mismatched parens") != &OpWrapper::Paren(Paren::Left) {
                            output.push(opstack.pop().unwrap().to_token());
                        }

                        opstack.pop();
                    }
                };

            },
        }
    }

    while !opstack.is_empty() {
        match opstack.pop().unwrap() {
            OpWrapper::Paren(_) => {
                panic!("Mismatched parens!");
            },
            a => {
                output.push(a.to_token())
            }
        }
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

}