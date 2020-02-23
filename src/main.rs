pub mod tree;
pub mod token;
pub mod function;
pub mod interpreter;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    match interpreter.begin() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };
}
