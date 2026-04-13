use clap::Parser;
use std::process::Command;
use std::fs;

// A basic downloader which uses your local IN PATH ENV youtube-dlp executable

/// A basic downloader which uses your local IN PATH ENV youtube-dlp executable
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


fn main() {
	// read txt file of links into a vector
	// let contents = fs::read_to_string("./links.txt").expect("Error reading file!");
	let args = Args::parse();
	let inputfile = args.input;
	let outputdir = args.output;
	let contents = fs::read_to_string(inputfile).expect("Error reading file!");
	let lines = contents.lines().collect::<Vec<_>>();

	//./output/
	// count for later
	let linknum = lines.len();
	let mut counter = 1;
	for link in lines {
		let outPutPath = outputdir.to_owned() + "/%(uploader)s - %(title)s.%(ext)s";
		println!("({counter}/{linknum}) Downloading: {link}");
		// for each link, run a specified youtube-dlp command
		// youtube-dlp -f "ba+bv/b" -o "%(uploader)s - %(title)s.%(ext)s" LINK
		Command::new("youtube-dlp")
			.arg("-f")
			.arg("ba+bv/b")
			.arg("-o")
			.arg(outPutPath)
			.arg(link)
			.output()
			.expect("Error trying to run youtube-dlp!");
		counter += 1;
	}
}
