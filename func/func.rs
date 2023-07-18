// create readfile function with parameter file (sting) as filename
fn fnReadfile(file: &str) {
	let chContent = std::fs::read_to_string(file).expect("Problem reading file.");
	// print the contents
	println!("{chContent}");
}

fn main() {
	// read the file text.txt
	fnReadfile("./text.txt");
}