use std::{
    io::{ Error, ErrorKind, },
    fmt::{ Display, Debug, Formatter, Result as fmt_Result, },
};

fn precedence(op: &Operator) -> usize {
    match op {
        Operator::Add => { 2 },
        Operator::Sub => { 2 },
        Operator::Mul => { 3 },
        Operator::Div => { 3 },
        Operator::Pow => { 4 },
        Operator::Mod => { 4 },
    }
}

#[derive(Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
} impl Operator {
    fn from_char(c: &char) -> Result<Self, Error> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '/' => Ok(Operator::Div),
            '*' => Ok(Operator::Mul),
            '^' => Ok(Operator::Pow),
            '%' => Ok(Operator::Mod),
            o => Err(Error::new(ErrorKind::Other, format!("Encountered unexpected character `{}`", o))),
        }
    }
    
    fn to_char(&self) -> char {
        match self {
            Operator::Add => '+', 
            Operator::Sub => '-', 
            Operator::Div => '/', 
            Operator::Mul => '*', 
            Operator::Pow => '^', 
            Operator::Mod => '%', 
        }
    }

    fn eval(&self, lhs: usize, rhs: usize) -> Result<usize, Error> {
        match self {
            Operator::Pow => { Ok(rhs.pow(lhs as u32)) },
            Operator::Add => { Ok(lhs + rhs) },
            Operator::Sub => { Ok(lhs - rhs) },
            Operator::Mul => { Ok(lhs * rhs) },
            Operator::Mod => { 
                if lhs == 0 {
                    Err(Error::new(ErrorKind::Other, "Unable to modulo by zero"))
                } else {
                    Ok(rhs % lhs)
                }
            },
            Operator::Div => { 
                if lhs == 0 {
                    Err(Error::new(ErrorKind::Other, "Unable to divide by zero"))
                } else {
                    Ok(rhs / lhs) 
                }
            },
        }
    }
}

#[derive(Clone)]
enum Token {
    Number(usize),
    Operator(Operator),
    Paren(bool),
} impl Token {
    fn operator_from_char(c: &char) -> Result<Self, Error> {
        Ok(Token::Operator(Operator::from_char(c)?))
    }

    fn number_from_str(s: &String) -> Result<Self, Error> {
        match s.parse::<usize>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Error getting number from string: {}", e)))
        }
    }

    fn paren_from_char(c: &char) -> Result<Self, Error> {
        match c {
            '(' => Ok(Token::Paren(true)),
            ')' => Ok(Token::Paren(false)),
            o => Err(Error::new(ErrorKind::Other, format!("Unable to create paren from non paren character `{}`", o)))
        }
    }

} impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        match self {
            Token::Operator(op) => write!(f, "{}", op.to_char()),
            Token::Number(num) => write!(f, "{}", num),
            Token::Paren(left) => {
                match left {
                    true => write!(f, "("),
                    false => write!(f, ")"),
                }
            }
        }
    }
} impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
struct TokenStream {
    tokens: Vec<Token>,
} impl TokenStream {
    fn new() -> Self {
        Self {
            tokens: Vec::new()
        }
    }

    fn add_token(&mut self, token: &Token) {
        self.tokens.push(token.clone())
    }

    fn from_string(s: &mut String) -> Result<Self, Error> {
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
                        // if the character is not a number, but exists
                        Some(x) => {
                            // if the character is not an operator or paren, return an error
                            if Token::paren_from_char(&x).is_err() && Operator::from_char(&x).is_err() {
                                // check if the character is whitespace, if so, skip the error and move on
                                if !x.is_whitespace() {
                                    return Err(Error::new(ErrorKind::Other, format!("ErrNo 1: Encountered unexpected character: `{}`", x)));
                                } 
                            }
                            // add the buffer (number) to the stream
                            stream.add_token(&Token::number_from_str(&buf.clone())?);
                            // clear the buffer
                            buf = String::new();
                            // if the current character is not whitespace (ex: "(10 + 1) - 2") -> " 1)", (the paren)
                            // add the character to the stream
                            if !x.is_whitespace() {
                                // Parens
                                if Token::paren_from_char(&x).is_ok() {
                                    stream.add_token(&Token::paren_from_char(&x)?);
                                }
                                // Operators
                                else if Operator::from_char(&x).is_ok() {
                                    stream.add_token(&Token::operator_from_char(&x)?)
                                }
                                // Otherwise, its not a character we want, return an error
                                else {
                                    return Err(Error::new(ErrorKind::Other, format!("ErrNo 2: Encountered unexpected character: `{}`", x)));
                                }
                            }
                            // break out of the seach loop
                            break 'num_search;
                            
                        },
                        // if there are no more characters in the iterator, break out of the loop
                        _ => break 'num_search
                    }
                }
            // if the character is an operator, add it to the stream
            } else if Operator::from_char(&c).is_ok() {
                stream.add_token(&Token::operator_from_char(&c)?);
            }
            // if the character is a paren, add it to the stream
            else if Token::paren_from_char(&c).is_ok() {
                stream.add_token(&Token::paren_from_char(&c)?);
            }
            // if we got here, this character doesn't belong
            else {
                return Err(Error::new(ErrorKind::Other, format!("ErrNo 3: Encountered unexpected character: `{}`", c)));
            }
        }

        if !buf.is_empty() {
            // if the buffer holds a number, add the buffer to the stream
            if buf.clone().chars().all(|c| c.is_numeric()) {
                stream.add_token(&Token::number_from_str(&buf)?);
            } else {
                // This should be unreachable
                return Err(Error::new(ErrorKind::Other, format!("ErrNo 4: Buffer not empty when it should be. Buffer contents: `{}`", buf)));
            }
        }

        return Ok(stream);
    }

} impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        let v = self.clone().tokens;
        let mut out = String::new();
        let last_index = if v.len() != 0 { v.len() - 1 } else { 0 };
        for (index, item) in v.into_iter().enumerate() {
            if index == last_index {
                out.extend(format!("{}", item).chars());
            } else {
                out.extend(format!("{} ", item).chars());
            }
        }

        write!(f, "{}", out)
    }
}

fn shunting_yard(stream: &TokenStream) -> Result<TokenStream, Error> {
    let mut output = TokenStream::new();

    let mut opstack = Vec::<Token>::new();

    let mut tokens = stream.tokens.clone().into_iter();

    while let Some(token) = tokens.next() {

        match token {
            // if the token is a number
            Token::Number(num) => {
                // push it to the output
                output.add_token(&Token::Number(num));
            },
            // if the token in an operator
            Token::Operator(op) => {
                //while  
                // (
                //     (there is an operator at the top of the operator stack with greater precedence)
                //     or
                //     (the operator at the top of the operator stack has equal precedence and is left associative)
                // )
                // and
                // (the operator at the top of the operator stack is not a left parenthesis):
                // pop operators from the operator stack to the output
                
                'popstack: loop {
                    match opstack.last() {
                        // if there is a token on the stack
                        Some(top) => {
                            match top {
                                // if it's a left paren
                                Token::Paren(true) => {
                                    break 'popstack
                                },
                                Token::Operator(stack_top) => {
                                    match stack_top {
                                        // if it's right associative
                                        Operator::Pow => {
                                            // if the top of the stack's precedence is greater than the current operator's
                                            // pop the top of the operator stack to the output
                                            if precedence(&Operator::Pow) > precedence(&op) {
                                                output.add_token(&opstack.pop().unwrap());
                                            }
                                            // otherwise, break the loop
                                            else {
                                                break 'popstack;
                                            }
                                        },
                                        // if it's left associative
                                        t => {
                                            // if the precedence of the top of the stack is equal to the current operator's,
                                            // pop the top of the operator stack to the output
                                            if precedence(&t) == precedence(&op) {
                                                output.add_token(&opstack.pop().unwrap());
                                            }
                                            // otherwise break the loop 
                                            else {
                                                break 'popstack;
                                            }
                                        }
                                    }
                                },
                                // if the token is a number somehow? return an error
                                Token::Number(_) => {
                                    return Err(
                                        Error::new(
                                            ErrorKind::Other,
                                            format!("I dont know how you got a number on the operator stack, but cool, \
                                                    here's a peek at the stack and output: Stack: {:?}; output: {}", 
                                                    opstack, output
                                            )
                                        )
                                    );
                                },
                                _ => { break 'popstack; }
                            }
                        },
                        None => { break 'popstack }
                    }
                }

                // after loop: push the operator to the operator stack
                opstack.push(Token::Operator(op));
            },
            // if the token is a paren 
            Token::Paren(left) => {
                match left {
                    // if it's a left paren
                    true => {
                        // push it to the operator stack
                        opstack.push(Token::Paren(true));
                    },
                    // if it's a right paren
                    false => {
                        // while the top of the operator stack is not a left paren pop the top of the operator stack to the output
                        loop {
                            match opstack.pop() {
                                Some(top) => {
                                    match top {
                                        // if the top of the stack is a left paren, pop it from the stack and discard, then break the loop
                                        Token::Paren(true) => {
                                            break;
                                        },
                                        _ => {
                                            output.add_token(&top);
                                        }
                                    }
                                },
                                // if the operator stack empties, you have mismatched parens
                                None => { return Err(Error::new(ErrorKind::Other, "ErrNo 6: Mismatched Parens!"))}
                            }
                        }
                    }
                }
            }
        }

    }

    // while there are operators on the stack, pop them to output
    while let Some(op) = opstack.last() {
        match op {
            // if the token is a paren, you have mismatched parens
            Token::Paren(_) => {
                return Err(Error::new(ErrorKind::Other, "ErrNo 7: Mismatched Parens!"));
            },
            _ => {
                output.add_token(&op);
                opstack.pop();
            }
        }
    }


    return Ok(output);
}

fn eval_rpn(stream: &TokenStream) -> Result<usize, Error> {

    let mut tokens = stream.tokens.clone().into_iter();
    let mut stack: Vec<usize> = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Operator(op) => {
                
                let x = match stack.pop() {
                    Some(n) => n,
                    None    => return Err(Error::new(ErrorKind::Other, "Unbalanced operation"))
                };

                let y = match stack.pop() {
                    Some(n) => n,
                    None    => return Err(Error::new(ErrorKind::Other, "Unbalanced operation"))
                };
                
                stack.push(op.eval(x, y)?);
            },
            Token::Number(num) => {
                stack.push(num);
            },
            _ => {
                ()
            }
        }
    }

    Ok(stack.first().unwrap().to_owned())
}

pub fn eval_string(string: &String) -> Result<usize, Error> {
    Ok(eval_rpn(&shunting_yard(&TokenStream::from_string(&mut string.clone())?)?)?)
}