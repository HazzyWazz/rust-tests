use std::fs;
use std::io::Write; 

fn download(link: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
	let response = reqwest::blocking::get(link)?.bytes()?;
	// write file to output directory
	let mut outputfile = fs::File::create(format!("./output/{filename}"))?;
	outputfile.write(&response)?;
	Ok(())
}

fn main() {
	// read file into strings
	let contents = fs::read_to_string("./links.txt").expect("Error reading file!");

	// separate each link in the file
	let lines = contents.lines();

	// for each link...
	for link in lines {
		// get filename and extension
		let filename = &link[link.rfind("/").unwrap() ..];
		println!("{filename}");
		// GET data from file url
		let _ = download(link, filename);
	}
}