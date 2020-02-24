
use std::{
    str::{ FromStr },
    // string::{ ToString },
    fmt::{ Debug, Display, Formatter, Result as fmt_Result },
};

use crate::function::Function;
use crate::interpreter::Context;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Var,
    Function,
} impl Keyword {
    fn from_str(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "var" | "let" => Some(Self::Var),
            "fn" | "func" | "function" => Some(Self::Function),
            _ => None
        }
    }
}impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", match self {
            Self::Var => "var",
            Self::Function => "function",
        })
    }
} 

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    USub,
    Assign,
} impl Operator {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
            '#' => Some(Self::USub),
            '=' => Some(Self::Assign),
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
            Self::USub => '#',
            Self::Assign => '=',
        }
    }

    pub fn evaluate(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => {
                if right == 0.0 {
                    panic!("Divide by zero");
                } else {
                    left / right
                }
            },
            Self::Pow => left.powf(right),
            Self::USub => -right,
            Self::Assign => panic!("evaluate shouldn't be called on Operator::Assign"),
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

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Operator(Operator),
    Value(f64),
    Paren(Paren),
    Keyword(Keyword),
    Function(Function),
    Identifier(String),
    Data(String),
} impl Token {
    pub fn new(literal: &str) -> Self {
        match literal.parse::<Token>() {
            Ok(t) => t,
            Err(e) => panic!(e)
        }
    }

} impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Token, Self::Err> {
        if let Ok(v) = s.parse::<f64>() {
            Ok(Token::Value(v))
        } else if let Ok(op) = s.parse::<Operator>() {
            Ok(Token::Operator(op))
        } else if let Ok(p) = s.parse::<Paren>() {
            Ok(Token::Paren(p))
        } else {
            Err(format!("Unexpected literal {}", s))
        }
    } 


} impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", {
            match self {
                Token::Operator(op) => op.to_string(),
                Token::Paren(p) => p.to_string(),
                Token::Value(v) => v.to_string(),
                Token::Keyword(kw) => format!("{}", kw),
                Token::Function(f) => format!("{}", f),
                Token::Data(d) => format!("DATA_TOK: {:?}", d),
                Token::Identifier(id) => format!("{}", id),
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


pub fn tokenize(s: &str, ctx: &mut Context) -> Result<Vec<Token>, String> {

    let mut nbuffer = String::new();
    let mut idbuffer = String::new();
    let mut idx = 0;
    let mut decl = Declaration::None;
    let mut tokens = Vec::new();
    let mut fnident = String::new();
    let mut argidents: Vec<String> = Vec::new();
    let mut varident = String::new();

    while let Some(c) = s.chars().nth(idx) {


        // check for unary operators (will always be first or directly following another operator (thanks greg!))
        // unwrap or will make this evalute true if it's the first item in the expression
        match tokens.last().unwrap_or(&Token::Operator(Operator::Add)) {
            Token::Operator(_) | Token::Paren(Paren::Left) => {
                if nbuffer.is_empty() && c == '-' {
                    tokens.push(Token::Operator(Operator::USub));
                    idx += 1;
                    continue;
                }
            },
            _ => ()
        }



        // put numbers into the buffer
        if c.is_numeric() || c == '.' {
            if c == '.' && nbuffer.contains('.') {
                return Err(format!("Encountered unexpected '.' when parsing {}", nbuffer));
            } else {
                nbuffer.push(c);
            }
        }
        // make a token from the buffer, add it, clear the buffer
        else if !nbuffer.is_empty() {
            match nbuffer.parse::<f64>() {
                Ok(f) => {
                    tokens.push(Token::Value(f))
                },
                Err(e) => unreachable!()
            }
            nbuffer = String::new();
        }
        // put characters in the identifier buffer
        else if c.is_ascii_alphabetic() {
            idbuffer.push(c);
        }
        // make a declaration type from the buffer, handle the rest of the input accordingly
        else if !idbuffer.is_empty() {
            if let Some(kw) = Keyword::from_str(idbuffer.clone()) {
                match kw {
                    Keyword::Function => decl = Declaration::Function,
                    Keyword::Var => decl = Declaration::Variable,
                }
                idbuffer = String::new();
            } else {
                match decl {
                    Declaration::Function => {
                        // if fnident is none, assign it,
                        if fnident.is_empty() {
                            fnident = idbuffer;
                            idbuffer = String::new();
                        }
                        // otherwise, check with argidents,
                        else {
                            // if the identifier is already there, add the same token again,
                            if argidents.contains(&idbuffer) {
                                tokens.push(Token::Identifier(idbuffer));
                                idbuffer = String::new();
                            }
                            // otherwise, add it to the argidents
                            else {
                                // if we're past the equals sign, raise a variableundefined error
                                if tokens.contains(&Token::Operator(Operator::Assign)) {
                                    return Err(format!("Encountered undefined variable {:?}", idbuffer));                                }
                                else {
                                    argidents.push(idbuffer);
                                    idbuffer = String::new();
                                }
                            }
                        }
                        // these will become the `Function`s `local_idents`
                    },
                    Declaration::Variable => {
                        // if varident is none, assign it,
                        if varident.is_empty() {
                            varident = idbuffer;
                            idbuffer = String::new();
                        }
                        // otherwise, check with the already assigned functions and variables to see if it exists already,
                        else if let Some(func) = ctx.functions.get(&idbuffer) {
                            tokens.push(Token::Function(func.clone()));
                            idbuffer = String::new();
                        } else if argidents.contains(&idbuffer) || ctx.variables.contains_key(&idbuffer) {
                            tokens.push(Token::Identifier(idbuffer));
                            idbuffer = String::new();
                        }
                        // if not, return a variableundefined error.
                        else {
                            return Err(format!("Encountered undefined variable {:?}", idbuffer));
                        }
                    },
                    Declaration::None => {
                        // check if the identifier is registered, if not, raise a variableundefined error.
                        if argidents.contains(&idbuffer) || ctx.variables.contains_key(&idbuffer) {
                            tokens.push(Token::Identifier(idbuffer));
                            idbuffer = String::new();
                        }
                    }
                }
            }
        }
        // check for operators, parens
        else if let Some(op) = Operator::from_char(c) {
            tokens.push(Token::Operator(op));
        }
        else if let Some(p) = Paren::from_char(c) {
            tokens.push(Token::Paren(p));
        }
        // increment the pointer
        idx += 1;    
    }

    eprintln!("tokens: {:?}, idbuffer: {}, nbuffer: {}, decl: {:?}", tokens, idbuffer, nbuffer, decl);

    // check the buffers for missed tokens
    if !idbuffer.is_empty() {
        if let Some(func) = ctx.functions.get(&idbuffer) {
            tokens.push(Token::Function(func.clone()));
        } else if ctx.variables.contains_key(&idbuffer) || argidents.contains(&idbuffer) {
            tokens.push(Token::Identifier(idbuffer.clone()));
        }
    } 

    
    if !nbuffer.is_empty() {
        match nbuffer.parse::<f64>() {
            Ok(f) => {
                tokens.push(Token::Value(f))
            },
            Err(e) => unreachable!()
        }
    }
    eprintln!("tokens: {:?}, idbuffer: {}, nbuffer: {}, decl: {:?}", tokens, idbuffer, nbuffer, decl);
    
    
    Ok(tokens)
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
                Operator::Assign => 6,
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

#[derive(Debug)]
enum Declaration {
    Function,
    Variable,
    None,
}


/// Takes an infix notated token stream and converts it to postfix notation
pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    // /*DEBUG:*/ eprintln!("Begin reverse poilsh conversion");
    let mut output: Vec<Token> = Vec::new();
    let mut opstack: Vec<Token> = Vec::new();
    
    for token in tokens {
        // /*DEBUG:*/ eprintln!("\nCurrent state:\n\tOperator stack: {:?}\n\tOutput: {:?}", opstack, output);
        // /*DEBUG:*/ eprint!("Encountered {:?} -> ", token);
        match token {
            Token::Value(_v) => {
                // /*DEBUG:*/ eprintln!("pushing token with value {} to the output", _v);
                output.push(token);
            },
            Token::Operator(_op) => {
                let p = precedence(&token);
                // /*DEBUG:*/ eprintln!("Operator {:?} -> Popping tokens from stack: ", _op);
                while !opstack.is_empty() {
                    match opstack.last() {
                        Some(&Token::Paren(_)) => {
                            // /*DEBUG:*/ eprintln!("\tEncountered paren, breaking");
                            break
                        },
                        Some(o) => {
                            // /*DEBUG:*/ eprint!("\tEncountered operator {} -> ", o);
                            if match OperatorAssociativity::from(&token) {
                                OperatorAssociativity::Left => {
                                    // /*DEBUG:*/ eprint!("looking for precedence({}) < {}...", o, p);
                                    precedence(o) < p
                                },
                                OperatorAssociativity::Right => {
                                    // /*DEBUG:*/ eprint!("looking for precedence({}) <= {}...", o, p);
                                    precedence(o) <= p
                                },
                            } {
                                // /*DEBUG:*/ eprintln!("Found! Breaking");
                                break
                            } else {
                                // /*DEBUG:*/ eprintln!("Not found, popping operator from the stack to the output");
                                output.push(opstack.pop().unwrap());
                            }
                        },
                        _ => unreachable!()
                    }
                }
                opstack.push(token.clone());
            },
            Token::Paren(p) => {
                // /*DEBUG:*/ eprint!("Encountered paren -> ");
                match p {
                    Paren::Left => {
                        // /*DEBUG:*/ eprintln!("Left paren, push to operator stack");
                        opstack.push(token.clone())
                    },
                    Paren::Right => {
                        // /*DEBUG:*/ eprintln!("Right paren, popping operator stack to output until we see a left paren");
                        while !opstack.is_empty() {
                            if let Some(top) = opstack.pop() {
                                match top {
                                    Token::Paren(Paren::Left) => {
                                        // /*DEBUG:*/ eprintln!("Encountered left paren, breaking");
                                        break
                                    },
                                    o => {
                                        // /*DEBUG:*/ eprintln!("\tpopping {} to the output", o);
                                        output.push(o)
                                    },
                                }
                            } else {
                                unreachable!()
                            }
                        }
                    },
                }
            },
            Token::Function(f) => {

            },
            _ => unreachable!()
        }

    }

    // /*DEBUG:*/ eprintln!("Clearing operator stack");
    while let Some(top) = opstack.pop() {
        // /*DEBUG:*/ eprintln!("Popping {} to output", top);
        output.push(top);
    }

    // /*DEBUG:*/ eprintln!("\nEnd reverse poilsh conversion\n");

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
    eprintln!(" left: {:?}\nright: {:?}", tokens, tokenize("(10+5)", &mut Context::new()).unwrap());
    assert!(tokens == tokenize("(10+5)", &mut Context::new()).unwrap());

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
    assert!(tokens == tokenize("((10.0 * 2) / 4 + (2.5 * 4) * 2)", &mut Context::new()).unwrap());

    // No parens
    let tokens = vec![
        Token::new("10"),
        Token::new("+"),
        Token::new("5"),
    ];
    assert!(tokens == tokenize("10.0 + 5", &mut Context::new()).unwrap());

    // Unary minus
    let tokens = vec![
        Token::new("#"),
        Token::new("10"),
        Token::new("+"),
        Token::new("#"),
        Token::new("5"),
    ];
    assert!(tokens == tokenize("-10 + -5", &mut Context::new()).unwrap());
    
}   

#[test]
fn test_shunting_yard() {
    let tokens = tokenize("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3", &mut Context::new()).unwrap();
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

    let tokens = tokenize("((15 / (7 -(1 + 1))) * 3) - (2 + (1 + 1))", &mut Context::new()).unwrap();
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

    // unary minus
    let tokens = tokenize("-10 + 5", &mut Context::new()).unwrap();
    let expected = vec![
        Token::new("10"),
        Token::new("#"),
        Token::new("5"),
        Token::new("+"),
    ];
    assert_eq!(shunting_yard(tokens), expected);

}

#[test]
fn test_operator_evaluate() {

    assert_eq!(Operator::Add.evaluate(1.0 , 10.0), 1.0  + 10.0);
    assert_eq!(Operator::Add.evaluate(15.0, 15.0), 15.0 + 15.0);
    assert_eq!(Operator::Add.evaluate(10.0, 20.0), 10.0 + 20.0);

    assert_eq!(Operator::Sub.evaluate(1.0 , 10.0), 1.0  - 10.0);
    assert_eq!(Operator::Sub.evaluate(15.0, 15.0), 15.0 - 15.0);
    assert_eq!(Operator::Sub.evaluate(10.0, 20.0), 10.0 - 20.0);

    assert_eq!(Operator::Mul.evaluate(1.0 , 10.0), 1.0  * 10.0);
    assert_eq!(Operator::Mul.evaluate(15.0, 15.0), 15.0 * 15.0);
    assert_eq!(Operator::Mul.evaluate(10.0, 20.0), 10.0 * 20.0);

    assert_eq!(Operator::Div.evaluate(1.0 , 10.0), 1.0  / 10.0);
    assert_eq!(Operator::Div.evaluate(15.0, 15.0), 15.0 / 15.0);
    assert_eq!(Operator::Div.evaluate(10.0, 20.0), 10.0 / 20.0);

    assert_eq!(Operator::Pow.evaluate(1.0 , 10.0), (1.0 as f64).powf(10.0));
    assert_eq!(Operator::Pow.evaluate(15.0, 15.0), (15.0 as f64).powf(15.0));
    assert_eq!(Operator::Pow.evaluate(10.0, 20.0), (10.0 as f64).powf(20.0));

    assert_eq!(Operator::USub.evaluate(0.0 , 10.0), -10.0);
    assert_eq!(Operator::USub.evaluate(0.0, 15.0), -15.0);
    assert_eq!(Operator::USub.evaluate(0.0, 10.0), -10.0);

}