use clap::Parser;
use std::fs;
use std::io::Write; 
use std::thread;
use std::time;

/// A simple direct link donwloader
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Input file
	#[arg(short, long)]
	input: String,

	/// Output directory (default = ./output)
	#[arg(short, long, default_value_t = String::from("./output"))]
	output: String
}

fn download(link: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
	let response = reqwest::blocking::get(link)?.bytes()?;
	// write file to output directory
	let mut outputfile = fs::File::create(format!("./output/{filename}"))?;
	outputfile.write(&response)?;
	Ok(())
}


fn main() {
	let args = Args::parse();

	let inputfile = args.input;
	let outputdir = args.output;

	println!("Input file: {inputfile}");
	println!("Output file: {outputdir}");

	let contents = fs::read_to_string(inputfile).expect("Error reading file!");
	// let lines = contents.lines();
	let lines = contents.lines().collect::<Vec<_>>();

	let mut counter = 1;
	let linknum = lines.len();

	for link in lines {
		// get filename and extension
		let filename = &link[link.rfind("/").unwrap() ..];
		println!("({counter}/{linknum}) Downloading: {filename}");
		// GET data from file url
		let _ = download(link, filename);
		// avoid MOST ratelimits
		thread::sleep(time::Duration::from_millis(1500));
		counter += 1;
	}
}