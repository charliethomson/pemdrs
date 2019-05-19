
use std::collections::{VecDeque, HashMap};
use std::iter::FromIterator;


#[derive(Hash, Eq, PartialEq)]
struct Token {
    s: String,
} impl Token {
    fn new(s: String) -> Token {
        Token { s }
    }

    fn from_char(c: char) -> Token {
        let s = String::new();
        s.push(c);
        Token { s }
    }

    fn as_chars(&self) -> Vec<char> {
        Vec::from_iter(self.s.chars())
    }

    fn eq_str(&self, o: &'static str) -> bool {
        self.s == String::from(o)
    }

    fn eq_char(&self, o: char) -> bool {
        self == &Token::from_char(o)
    }
    
    fn is_numeric(&self) -> bool {
        for c in self.s.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
} impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}


struct Expression {
    tokens: Vec<Token>,
} impl Expression {
    fn from_string(s: String) -> Expression {
        let ops = vec!['(', ')', '+', '-', '*', '/', '^'];
        let mut tokens = Vec::new();
        for c in s.chars() {
            if c.is_numeric() {

            } else if ops.contains(&c) {
                tokens.push(Token::from_char(c));
            }
            let last = c;
        };

        Expression { tokens }
    }

} 

fn vecdequeue_top<T>(vdq: &mut VecDeque<T>) -> Option<&T> {
    match vdq.len() {
        0 => None,
        n => Some(&vdq[n-1]),
    }
}


fn shunting_yard(expr: Expression) -> String {
    let ops: HashMap<char , u8> = HashMap::from_iter(vec![
        ('(', 0),
        (')', 0),
        ('+', 2),
        ('-', 2),
        ('*', 3),
        ('/', 3),
        ('^', 4),
    ]);
    let mut output:  VecDeque<&Token> = VecDeque::new();
    let mut opstack: VecDeque<&Token> = VecDeque::new();
    for c in expr.tokens.iter() {
        // if c is a number, push it to the output queue
        if c.is_numeric() {
            output.push_back(c);
        }
        // if c is an operator
        else if c.is_operator() {
            // if c is a paren
            if c.is_paren() {
                if c.eq_char('(') {
                    opstack.push_back(c);
                } else { /* closinchars()g paren */

                }
            }
            // end parens
            // other operators 
            else {
                eprintln!("{:?}", opstack);
                let precedence = ops.get(&c).unwrap();

                while ((ops.get(opstack.get(opstack.len()).unwrap()).unwrap() > precedence)
                || ops.get(opstack.get(opstack.len()).unwrap()).unwrap() == precedence && c != '^')
                && c != '(' {
                    eprintln!("{:?}", opstack);
                    output.push(opstack.pop_back().unwrap());
                }
                opstack.push_back(c);
            }
        } 
    };
    return output;
}

fn main() {
    eprintln!("{}", shunting_yard(String::from("(10+4)")));
}
