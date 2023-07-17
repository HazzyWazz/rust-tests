use std::fs;

fn main() {
	let contents = fs::read_to_string("./text.txt").expect("Crazy? I was crazy once. They put me in a room. A rubber room. A rubber room with rats. And rats make me crazy. Crazy? I was crazy once... (Error reading file)");
	println!("{contents}");
}