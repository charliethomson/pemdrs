/*
IDEA:
    `token`: any item in an expression; can be a number, function, or operator
    `expression`: wrapper for Vec<Token>; works on tokens in bulk

*/

use std::{
    alloc::{ alloc_zeroed, dealloc, realloc },

};

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
        ['(', ')', '+', '-', '*', '/', '^'].contains(&self.as_char())
    }

    fn is_paren(&self) -> bool {
        ['(', ')'].contains(&self.as_char())
    }


}