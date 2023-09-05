use clap::Parser;

/// Simple argument parse prototype for downloader
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
	let args = Args::parse();

	let inputfile = args.input;
	let outputdir = args.output;

	println!("Input file: {inputfile}");
	println!("Output file: {outputdir}");
}