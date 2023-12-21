use bf_interpreter::BfInterpreter;
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("Failed to read file")
    } else {
        println!("Enter your Brainfuck program and press Ctrl+D (Unix) or Ctrl+Z (Windows) when done:");
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
        input
    };

    let mut interpreter = BfInterpreter::new();
    interpreter.execute(&program);
}