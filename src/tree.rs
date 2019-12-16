
use crate::token::{ Token, Operator, Paren};
use std::str::FromStr;


#[derive(Clone, Copy)]
struct ArrayNode {
    token: Token,
    left: Option<usize>,
    right: Option<usize>,
} impl ArrayNode {
    fn new(token: Token) -> Self {
        ArrayNode {
            token,
            left: None,
            right: None,
        }
    }

    fn evaluate(&self, nodes: &Vec<ArrayNode>) -> f64 {
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
} impl std::fmt::Debug for ArrayNode {
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

struct ArrayTree {
    nodes: Vec<ArrayNode>, /* root idx 0 */
} impl ArrayTree {
    fn new() -> Self {
        ArrayTree { nodes: Vec::new() }
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

        eprintln!("Time taken: {:?} ns", start.elapsed().as_nanos());
    }}
}

struct LinkedNode<'a> {
    token: Token,
    left: Option<&'a LinkedNode<'a>>,
    right: Option<&'a LinkedNode<'a>>,
} impl<'a> LinkedNode<'a> {
    fn new(token: Token) -> Self {
        LinkedNode {
            token,
            left: None,
            right: None,
        }
    }
    
    fn evaluate(&self) -> f64 {
        match self.token {
            Token::Operator(op) => {
                let l = if let Some(left) = self.left {
                    left.evaluate()
                } else {
                    0.0
                };

                let r = if let Some(right) = self.right {
                    right.evaluate()
                } else {
                    0.0
                };

                op.evaluate(l, r)
            },
            Token::Value(v) => v,
            Token::Paren(_) => panic!("this shouldnt happen")
        }
    }
}

struct LinkedTree<'a> {
    root: LinkedNode<'a>,
} impl<'a> LinkedTree<'a> {
    fn new(root: LinkedNode<'a>) -> Self {
        LinkedTree { root }
    }

    fn evaluate(&self) -> f64 {
        self.root.evaluate()
    }
}

#[test]
fn test_evaluate() {
    // Test equation: -10+(2*10)+(-10)+(10*11)
    // Expected result: 110
    eprint!("\nManual allocation of centralized array tree: ");
    let mut t    = ArrayTree::new();
    let mut root = ArrayNode::new(Token::new("+"));
    let mut l    = ArrayNode::new(Token::new("+"));
    let mut ll   = ArrayNode::new(Token::new("-"));
    let mut llr  = ArrayNode::new(Token::new("10"));
    let mut lr   = ArrayNode::new(Token::new("*"));
    let mut lrl  = ArrayNode::new(Token::new("2"));
    let mut lrr  = ArrayNode::new(Token::new("10"));
    let mut r    = ArrayNode::new(Token::new("+"));
    let mut rl   = ArrayNode::new(Token::new("-"));
    let mut rlr  = ArrayNode::new(Token::new("10"));
    let mut rr   = ArrayNode::new(Token::new("*"));
    let mut rrl  = ArrayNode::new(Token::new("10"));
    let mut rrr  = ArrayNode::new(Token::new("11"));
    
    rr.left = Some(11);
    rr.right = Some(12);
    rl.right = Some(9);
    r.left = Some(8);
    r.right = Some(10);
    root.right = Some(7);
    lr.left = Some(5);
    lr.right = Some(6);
    ll.right = Some(3);
    l.right = Some(4);
    l.left = Some(2);
    root.left = Some(1);

    t.nodes = vec![
        root,   // +    0
        l,      // +    1
        ll,     // -    2
        llr,    // 10   3
        lr,     // *    4
        lrl,    // 2    5
        lrr,    // 10   6
        r,      // +    7   
        rl,     // -    8
        rlr,    // 10   9
        rr,     // *    10
        rrl,    // 10   11
        rrr,    // 11   12
    ];
    eprintln!("Done.");

    eprint!("Recursive centralized array test: ");
    timeit!({
        eprintln!("{}", t.evaluate());
    });


    eprint!("\nLinked tree initialization: ");
    let mut root = LinkedNode::new(Token::new("+"));
    let mut l = LinkedNode::new(Token::new("+"));
    let mut ll = LinkedNode::new(Token::new("-"));
    let mut llr = LinkedNode::new(Token::new("10"));
    let mut lr = LinkedNode::new(Token::new("*"));
    let mut lrl = LinkedNode::new(Token::new("2"));
    let mut lrr = LinkedNode::new(Token::new("10"));
    let mut r = LinkedNode::new(Token::new("+"));
    let mut rl = LinkedNode::new(Token::new("-"));
    let mut rlr = LinkedNode::new(Token::new("10"));
    let mut rr = LinkedNode::new(Token::new("*"));
    let mut rrl = LinkedNode::new(Token::new("10"));
    let mut rrr = LinkedNode::new(Token::new("11"));
    
    rr.left = Some(&rrl);
    rr.right = Some(&rrr);

    rl.right = Some(&rlr);

    r.left = Some(&rl);
    r.right = Some(&rr);

    root.right = Some(&r);

    lr.left = Some(&lrl);
    lr.right = Some(&lrr);

    ll.right = Some(&llr);
    l.left = Some(&ll);
    l.right = Some(&lr);
    root.left = Some(&l);

    let mut t = LinkedTree::new(root);

    eprintln!("Done.");

    eprint!("Linked test: ");

    timeit!({
        eprintln!("{}", t.evaluate());
    });

    eprintln!("");
}