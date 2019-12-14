
use crate::token::Token;

#[derive(Clone, Copy, PartialEq)]
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
}

#[derive(Copy, Clone, PartialEq)]
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
}

struct Node {
    token: Token,
    left: Option<usize>,
    right: Option<usize>,
} impl Node {
    fn new(token: Token) -> Self {
        Node {
            token,
            left: None,
            right: None,
        }
    }

    fn evaluate(&self, nodes: &Vec<Node>) -> f64 {
        match self.token {
            Token::Operator(op) => {
                let l = if let Some(left) = self.left {
                    nodes.get(left).expect("Failed to get left node").evaluate(nodes)
                } else {
                    0.0
                };

                let r = if let Some(right) = self.right {
                    nodes.get(right).expect("Failed to get right node").evaluate(nodes)
                } else {
                    0.0
                };

                op.evaluate(l, r)
            },
            Token::Value(v) => v,
            Token::Paren(_) => panic!("this shouldnt happen")
        }
    }
} impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.token {
            Token::Operator(op) => {
                if op == Operator::Sub && self.left.is_none() {
                    write!(f, "(-{:?})", self.right.unwrap())
                } else {
                    write!(f, "({:?} {} {:?})", self.left.unwrap(), op.to_char(), self.right.unwrap())
                }
            },
            Token::Value(v) => {
                write!(f, "{}", v)
            }
            Token::Paren(_) => panic!("this shouldnt happen")
        }
    }
}

struct Tree {
    nodes: Vec<Node>, /* root idx 0 */
} impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    fn evaluate(&self) -> f64 {
        return self.nodes.get(0)
                        .expect("Cannot evaluate empty tree")
                        .evaluate(&self.nodes);
    }
}

macro_rules! timeit {
    ($b:block) => {{
        use std::time::{ Instant, Duration };
        let start = Instant::now();

        $b

        let end = start.elapsed();
        eprintln!("Time taken: {:?} ms", end.as_millis());
    }}
}

struct LinkedNode<'a> {
    token: Token,
    left: Option<&'a Node>,
    right: Option<&'a Node>,
}

#[test]
fn test_evaluate() {
    let mut t = Tree::new();
    let mut root = Node::new(Token::new("+"));
    let mut l = Node::new(Token::new("+"));
    let mut ll = Node::new(Token::new("-"));
    let mut llr = Node::new(Token::new("10"));
    let mut lr = Node::new(Token::new("*"));
    let mut lrl = Node::new(Token::new("2"));
    let mut lrr = Node::new(Token::new("10"));
    let mut r = Node::new(Token::new("/"));
    let mut rl = Node::new(Token::new("-"));
    let mut rlr = Node::new(Token::new("10"));
    let mut rr = Node::new(Token::new("*"));
    let mut rrl = Node::new(Token::new("10"));
    let mut rrr = Node::new(Token::new("11"));
    t.nodes = vec![
        root,   // +
        l,      // +
        ll,     // -
        llr,    // 10
        lr,     // *
        lrl,    // 2
        lrr,    // 10
        r,      // /
        rl,     // -
        rlr,    // 10
        rr,     // *
        rrl,    // 10
        rrr,    // 11
    ];
    eprint!("Recursive centralized array test: ");
    timeit!({
        
    })
}