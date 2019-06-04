#![feature(type_alias_enum_variants)]
use std::{
    io::{ Error, ErrorKind, },
};

enum Paren {
    Left,
    Right,
} impl Paren {

    fn is_left(&self) -> bool {
        match self {
            Self::Left => true,
            _ => false,
        }
    }

    fn is_right(&self) -> bool {
        match self {
            Self::Right => true,
            _ => false,
        }
    }
    
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Paren(Paren),
} impl Operator {
    fn from_char(c: &char) -> Result<Self, Error> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '/' => Ok(Self::Mul),
            '*' => Ok(Self::Div),
            '%' => Ok(Self::Mod),
            '(' => Ok(Self::Paren(Paren::Left)),
            ')' => Ok(Self::Paren(Paren::Right)),
            o   => Err(Error::new(ErrorKind::Other, format!("Unexpected operator in Operator::from_char({})", o)))
        }
    }
}

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
}

enum Token {
    Number(Number),
    Operator(Operator),
} impl Token {
    
    fn number(num: usize) -> Self {
        Self::Number(Number::with_value(num))
    }

    fn number_from_str(s: &'static str) -> Result<Self, Error> {
        match s.parse::<usize>() {
            Ok(a) => Ok(Self::number(a)),
            Err(e) => Err(Error::new(ErrorKind::Other, format!("<Token>::number_from_str({}) error: {}", s, e)))
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }

    fn operator_from_char(c: &char) -> Result<Self, Error> {
        let opr = Operator::from_char(c)?;
        Ok(Self::operator(opr))
    }

    fn is_operator(&self) -> bool {
        match self {
            Self::Operator(_) => true,
            _ => false,
        }
    }
    
}

struct Scanner {

} impl Scanner {
    
}
