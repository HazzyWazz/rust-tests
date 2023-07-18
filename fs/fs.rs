use std::fs; // import fs

fn main() {
	// read file into contents
	let contents = fs::read_to_string("./text.txt").expect("Crazy? I was crazy once. They put me in a room. A rubber room. A rubber room with rats. And rats make me crazy. Crazy? I was crazy once... (Error reading file)");
	// print contents
	println!("{contents}");
}