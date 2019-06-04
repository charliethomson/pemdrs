use std::{
    io::{ Error, ErrorKind, },
    fmt::{ Debug, Display, Formatter, Result as fmt_Result, },
};

fn join<T: Display>(v: Vec<T>, sep: T) -> String {
    let mut out = String::new();
    let last_index = v.len() - 1;
    for (index, item) in v.into_iter().enumerate() {
        if index == last_index {
            out.extend(format!("{}", item).chars());
        } else {
            out.extend(format!("{}{}", item, sep).chars());
        }
    }
    return out;
}

#[derive(Clone)]
enum Paren {
    Left,
    Right,
} impl Paren {

    fn is_left(&self) -> bool {
        match self {
            Paren::Left => true,
            _ => false,
        }
    }

    fn is_right(&self) -> bool {
        match self {
            Paren::Right => true,
            _ => false,
        }
    }
    
}

#[derive(Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Paren(Paren),
} impl Operator {
    fn char_is_valid(c: &char) -> bool {
        match Self::from_char(c) {
            Ok(_) => true,
            _     => false,
        }
    }

    fn from_char(c: &char) -> Result<Self, Error> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '/' => Ok(Operator::Mul),
            '*' => Ok(Operator::Div),
            '%' => Ok(Operator::Mod),
            '(' => Ok(Operator::Paren(Paren::Left)),
            ')' => Ok(Operator::Paren(Paren::Right)),
            o   => Err(Error::new(ErrorKind::Other, format!("Unexpected operator in <Operator>::from_char({})", o)))
        }
    }

    fn as_str(&self) -> String {
        match self {

        
            Operator::Add => format!("{}", '+'),
            Operator::Sub => format!("{}", '-'),
            Operator::Mul => format!("{}", '/'),
            Operator::Div => format!("{}", '*'),
            Operator::Mod => format!("{}", '%'),
            Operator::Paren(Paren::Left) => format!("{}", '('),
            Operator::Paren(Paren::Right) => format!("{}", ')'),
        }
    }
} impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone)]
struct Number {
    value: usize,    
} impl Number
{
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn with_value(value: usize) -> Self {
        Self {
            value,
        }
    }

    fn get_value(&self) -> usize {
        self.value.clone()
    }

    fn set_value(&mut self, v: usize) {
        self.value = v;
    }

    fn as_str(&self) -> String {
        format!("{}", self.value)
    }
} impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone)]
enum Token {
    Number(Number),
    Operator(Operator),
    Whitespace,
} impl Token {
    
    fn whitespace() -> Self {
        Token::Whitespace
    }

    fn number(num: usize) -> Self {
        Token::Number(Number::with_value(num))
    }

    fn number_from_str(s: String) -> Result<Self, Error> {
        match s.parse::<usize>() {
            Ok(a) => Ok(Token::number(a)),
            Err(e) => Err(Error::new(ErrorKind::Other, format!("<Token>::number_from_str({}) error: {}", s, e)))
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false,
        }
    }

    fn operator(op: Operator) -> Self {
        Token::Operator(op)
    }

    fn operator_from_char(c: &char) -> Result<Self, Error> {
        let opr = Operator::from_char(c)?;
        Ok(Token::operator(opr))
    }

    fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_) => true,
            _ => false,
        }
    }
    
    fn as_str(&self) -> String {
        match self {
            Token::Whitespace => format!("{}", ' '),
            Token::Number(num) => num.as_str(),
            Token::Operator(op) => op.as_str(),
        }
    }
} impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", self.as_str())
    }
}

struct TokenStream {
    tokens: Vec<Token>,
} impl TokenStream {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    fn from_string(s: &String) -> Result<Self, Error> {
        let mut iter = s.chars();
        let mut buf = String::new();
        let mut stream = Self::new();

        while let Some(c) = iter.next() {
            // Skip whitespace
            if c.is_whitespace() {
                continue;
            } else if c.is_numeric() {
                // add the number to the buffer
                buf.push(c);
                // search for more digits
                'num_search: loop {
                    match iter.next() {
                        // if we find a number, add it to the buffer, and continue searching
                        Some(x) if x.is_numeric() => {
                            buf.push(x);
                        },
                        // if we find a different character (an operator), add the number to the stream,
                        // clear the buffer, and add the operator to the stream
                        Some(x) => {
                            // if the character is not an operator, return an error
                            if !Operator::char_is_valid(&x) && !x.is_whitespace() {
                                return Err(Error::new(ErrorKind::Other, format!("Encountered unexpected character: '{}'", x)));
                            }
                            stream.add_token(Token::number_from_str(buf.clone())?);
                            buf = String::new();
                            if !x.is_whitespace() {
                                stream.add_token(Token::operator_from_char(&x)?);
                            }
                            break 'num_search;
                        },
                        // otherwise, break the search (will exit the main loop, no chars left in the iterator)
                        _ => break 'num_search
                    }
                }
            // if the character is an operator
            } else if Operator::char_is_valid(&c) {
                stream.add_token(Token::operator_from_char(&c)?);
                buf = String::new();
            }
        }

        return Ok(stream);
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
} impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", join(self.tokens.clone(), Token::Whitespace))
    }
}



fn main() {
    let s = String::from("(10 + 11) - 3 / (4 * 11)");

    eprintln!("original: {};\nTokenStream: {};", &s, TokenStream::from_string(&s).unwrap());
}