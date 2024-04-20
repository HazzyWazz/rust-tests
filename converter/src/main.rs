use clap::Parser;
use std::fs;
use std::process::Command;

/// A converter to go with music, uses your local IN PATH ffmpeg install
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Input folder
	#[arg(short, long)]
	input: String,

	/// Output directory (default = ./output)
	#[arg(short, long, default_value_t = String::from("./output"))]
	output: String
}


fn main() {
	// read input folder of files into a vector (paths)
	//let input = fs::read_dir("./input").expect("Error reading input directory!").collect::<Vec<_>>();
	let args = Args::parse();
	let inputdir = args.input;
	let output = args.output;
	let input = fs::read_dir(inputdir).expect("Error reading input directory!").collect::<Vec<_>>();


	// counter for later
	let inputnum = input.len();
	let mut counter = 1;
	for paths in input {
		// for each FILE, read the path and convert it to string
		let path = paths.expect("Error reading path!").path();
		let pathstr = path.display().to_string();

		// grab the filename, there's probably an easier way
		let filename = String::from(path.file_name().expect("Unable to convert path to string!").to_str().unwrap());
		println!("({counter}/{inputnum}) Converting: {}", filename);

		// set output path crudely
		let outputpath = output.to_owned() + "/" + &filename + ".mp3";

		// run specific FFMPEG
		// ffmpeg -hide_banner -i INPUT -ac 2 -ar 48000 -ab 320k OUTPUT
		Command::new("ffmpeg")
			.arg("-hide_banner")
			.arg("-i")
			.arg(pathstr)
			.arg("-ac")
			.arg("2")
			.arg("-ar")
			.arg("48000")
			.arg("-ab")
			.arg("320k")
			.arg(outputpath)
			.output()
			.expect("Error converting WEBM to MP3!");
		counter += 1;
	}
}
