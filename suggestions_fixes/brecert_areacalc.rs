use std::io::{self, Write, Result};

/// Displays a message to `STDOUT`, then return a line from `STDIN` with whitespace trimmed from both sides
fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    // ensure the above is outputted before reading input, returns the error if there's an error flushing
    io::stdout().flush()?;

    let mut input = String::new();
    // read line from stdin into input, returning the error if there's an error reading the line
    io::stdin().read_line(&mut input)?;

    // remove whitespace from the input
    Ok(input.trim().to_string())
}

fn main() {
    let side_a: u64 = prompt("Length of side A (no units, positive numbers only): ")
        .expect("Error reading from STDIN")
        .parse()
        .expect("Unable to parse number");

    let side_b: u64 = prompt("Length of side B (no units, positive numbers only): ")
        .expect("Error reading from STDIN")
        .parse()
        .expect("Unable to parse number");

    let perimeter = side_a + side_b;
    let area = side_a * side_b;
    
    // print's Square if side_a is equal to side_b, otherwise print Rectangle
    println!("{}", if side_a == side_b { "Square" } else { "Rectangle" });
    println!("Perimeter: {perimeter}");
    println!("Area: {area}");
}

