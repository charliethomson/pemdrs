
use crate::token::{ Token, Operator, Paren};

use std::{
    fmt::{ Display, Debug, Formatter, Result as fmt_Result },
};


#[derive(Copy, Clone)]
struct Node {

} impl Node {
    fn new(token: Token) -> Self {
        Node {

        }
    }

    fn evaluate(&self) -> f64 {
        0.0
    }
} impl From<Token> for Node {
    fn from(token: Token) -> Self {
        Node::new(token)
    }

} impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "")
    }
}

#[derive(Copy, Clone)]
struct Tree {

} impl Tree {
    fn new() -> Self {
        Tree {
            
        }
    }

    fn evaluate(&self) -> f64 {
        0.0
    }
} impl From<Vec<Token>> for Tree {
    fn from(stream: Vec<Token>) -> Self {
        Tree {

        }
    }
} impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "")
    }
} impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "")
    }
} 




#[test]
fn test_node_from_token() {

}

#[test]
fn test_node_evaluate() {

}

#[test]
fn test_node_debug() {

}

#[test]
fn test_tree_from_vec_token() {

}

#[test]
fn test_tree_evaluate() {

}

#[test]
fn test_tree_debug() {
    
}

#[test]
fn test_tree_display() {
    
}