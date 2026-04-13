use std::fs;
use std::io::Write;

fn main() {
	let query = fs::read_to_string("./query2.txt").expect("Error reading checkFile!");
	let qlines = query.lines();

	let mut tracksCounter = 1;
	let mut tracksVector: Vec<&str> = Vec::new();

	for line in qlines {
		if line.contains("spotify:track:") {
			if tracksVector.contains(&line.trim()) != true {
				// println!("({}/106) | Adding {}", tracksCounter, line.trim());
				tracksVector.push(line.trim());
				tracksCounter += 1;
			}
		}
	}

	println!("Successfully yoinked {} tracks!", tracksCounter);
	println!("{:?}", tracksVector);

	let mut outFile = fs::File::create("./output.txt").expect("Unable to create file");
	for i in &tracksVector {
		println!("{}", i);
		let track = i.split(":").collect::<Vec<&str>>()[3].strip_suffix("\"").unwrap();
		// println!("Track: {}", track);
		writeln!(outFile, "{}", track);
		// dbg!(track);
	}
}