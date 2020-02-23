pub mod tree;
pub mod token;

use std::io::*;

fn main() {
    let mut input = std::io::stdin();
    println!("Enter your number thing");
    loop {
        let mut cmd = String::new();
        

        input.read_line(&mut cmd);

        
        println!("{}", match tree::evaluate(cmd) {
            Ok(v) => format!("{}", v),
            Err(e) => format!("Encountered an error while evaluating: {}", e) 
        });

    }
}
