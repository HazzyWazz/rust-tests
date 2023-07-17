use std::io::{self, Write};  // "import" stdio library

fn main() {  //create main
    let mut name = String::new();  // create new mutable (changeable) string `name` 
    print!("What's your name? ");  // print question to stdout
    let _ = io::stdout().flush();  // ...create a newline?  actually: flush makes sure all buffered content is outputted
    io::stdin().read_line(&mut name).expect("Error reading from STDIN");  // read input on stdin newline into `name`, throw error on error
    println!("Hello {}!", name.trim());  // print name (trimmed, whatever that means) to stdout
}