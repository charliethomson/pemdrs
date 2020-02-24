
use crate::token::{ Token, Operator, shunting_yard, tokenize };
use crate::interpreter::Context;
use std::{
    fmt::{ Display, Debug, Formatter, Result as fmt_Result },
    collections::HashMap,
};


#[derive(Clone)]
struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
} impl Node {
    fn new(token: Token) -> Self {
        Node {
            token,
            left: None,
            right: None,
        }
    }

    fn evaluate(&self) -> f64 {
        match self.token {
            Token::Value(v) => v,
            Token::Operator(op) => {
                match op {
                    Operator::USub => -self.right.as_ref().expect("Something went wrong! (evaluate unary minus without right child)").evaluate(),
                    _ => {
                        op.evaluate(
                            self.left.as_ref().expect("Something went wrong! (evaluate non unary operator node without left child").evaluate(),
                            self.right.as_ref().expect("Something went wrong! (evaluate non unary operator node without right child").evaluate(),
                        )
                    }
                }
            },
            _ => unreachable!()
        }
    }

    fn depth(&self) -> u16 {
        let l = match self.left.as_ref() {
            Some(node) => node.depth(),
            None       => 0,
        };
        let r = match self.right.as_ref() {
            Some(node) => node.depth(),
            None       => 0,
        };

        l.max(r) + 1
    }
} impl From<Token> for Node {
    fn from(token: Token) -> Self {
        Node::new(token)
    }

} impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        match self.token {
            Token::Value(v) => write!(f, "{}", v),
            Token::Operator(op) => {
                match op {
                    Operator::USub => write!(f, "u{:?}", self.right.as_ref().expect("Something went wrong! (format unary minus without right child)")),
                    _ => {
                        write!(
                            f, 
                            "({:?} {} {:?})", 

                            self.left.as_ref().expect("Something went wrong! (format operator node with no left child)"),
                            op.to_string(),
                            self.right.as_ref().expect("Something went wrong! (format operator node with no right child)"),
                        )
                    },
                }
            },
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
struct Tree {
    root: Node,
} impl Tree {
    fn new(s: &str) -> Result<Self, String> {
        Self::from_vec(shunting_yard(tokenize(s, &mut Context::new())?), &mut Context::new())
    }

    fn from_vec(stream: Vec<Token>, ctx: &mut Context) -> Result<Self, String> {
        let mut stack: Vec<Node> = Vec::new();

        for token in stream {
            match token {
                Token::Value(_) => stack.push(token.into()),
                Token::Operator(op) => {
                    match op {
                        Operator::USub => {
                            let mut node: Node = token.clone().into();
                            let value = match stack.pop() {
                                Some(v) => v,
                                None => return Err("You seem to have an unbalanced tree :(".to_owned())
                            };
                            node.right = Some(Box::new(value));
                            stack.push(node);
                        },
                        _ => {
                            let mut node: Node = token.clone().into();
                            let (a, b) = match (stack.pop(), stack.pop()) {
                                (Some(a), Some(b)) => (a, b),
                                _ => return Err("You seem to have an unbalanced tree :(".to_owned())

                            };
                            node.right = Some(Box::new(a));
                            node.left = Some(Box::new(b));
                            stack.push(node);
                        }
                    }
                },
                _ => unreachable!()
            }
        }

        match stack.pop() {
            Some(root) => Ok(Tree { root }),
            None => Err("Empty string? maybe? (stack empty)".to_owned())
        }
    }

    fn evaluate(&self) -> f64 {
        self.root.evaluate()
    }
} impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{:?}", self.root)
    }
} impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "")
    }
} 


#[test]
fn test_tree_evaluate() {

    std::thread::sleep_ms(100);

    let mut problems: HashMap<&str, f64> = HashMap::new();

    // Problems generated from https://www.math-aids.com/Order_of_Operations/Advanced_Order_of_Operations.html

    // easy
    problems.insert("-2--4--8+-2+-11", -3.0);
    problems.insert("-2*-11+-3*-7+-4",  39.0);
    problems.insert("48/4+7-6+6",       19.0);
    problems.insert("-12--10+-4+-6*-4", 18.0);
    problems.insert("5+-12-42/7*-10",   53.0);

    // medium
    problems.insert("(-2^3+-3)*-2--4+-3",  23.0);
    problems.insert("(54/9)^2-4*7+7",      15.0);
    problems.insert("(-3^2+-4)*-3--9+-4", -10.0);
    problems.insert("4-(8/4)^3*9+9",      -59.0);
    problems.insert("6-(10/5)^2*-5+-5",    21.0);

    // hard
    problems.insert("(5-(9/3)^2)*6+6",      -18.0);
    problems.insert("(10+(16/8))*3^3-8",     316.0);
    problems.insert("((4^2+-6)*4)-3+6",      43.0);
    problems.insert("(4-(-2^2-4))*(-2-8)",  -40.0);
    problems.insert("((-78/-13)^3-8)*-4+4", -828.0);
    

    
    for (problem, answer) in problems.iter() {

        // eprintln!("Evaluating {}; expected: {}", problem, answer);
        let tree = Tree::new(problem).unwrap();
        assert_eq!(&tree.evaluate(), answer);

        /* // DEBUG
        eprintln!(
            "Problem: {}, expected: {}\n\tparsed as: {:?}\n\tevaluated to: {}\n\tdepth: {} \n\t    ({} : {})\n", 
            problem,
            answer,
            tree.root,
            tree.evaluate(),
            tree.root.depth(),
            tree.root.left.as_ref().unwrap().depth(),
            tree.root.right.as_ref().unwrap().depth(),
        )
        */
    }
}

pub fn evaluate(tokens: &Vec<Token>, ctx: &mut Context) -> Result<f64, String> {
    match Tree::from_vec(tokens.clone(), ctx) {
        Ok(tree) => Ok(tree.evaluate()),
        Err(e) => Err(e)
    }
}