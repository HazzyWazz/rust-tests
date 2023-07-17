fn fnReadfile(file: &str) {
	let chContent = std::fs::read_to_string(file).expect("Problem reading file.");
	println!("{chContent}");
}

fn main() {
	fnReadfile("./text.txt");
}