use std::process::Command;
use std::fs;

// A basic downloader which uses your local IN PATH ENV youtube-dlp executable

fn main() {
	// read txt file of links into a vector
	let contents = fs::read_to_string("./links.txt").expect("Error reading file!");
	let lines = contents.lines().collect::<Vec<_>>();

	// count for later
	let linknum = lines.len();
	let mut counter = 1;
	for link in lines {
		println!("({counter}/{linknum}) Downloading: {link}");
		// for each link, run a specified youtube-dlp command
		// youtube-dlp -f "ba+bv/b" -o "%(uploader)s - %(title)s.%(ext)s" LINK
		Command::new("youtube-dlp")
			.arg("-f")
			.arg("ba+bv/b")
			.arg("-o")
			.arg("./output/%(uploader)s - %(title)s.%(ext)s")
			.arg(link)
			.output()
			.expect("Error trying to run youtube-dlp!");
		counter += 1;
	}
}
