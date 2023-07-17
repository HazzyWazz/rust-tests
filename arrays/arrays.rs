// python equivalent of lists
// let's try structs and
// let's parse some CSV type data

// I left in the struct for educational purposes

use std::fs::read_to_string;


fn readLines(filename: &str) -> Vec<String> {
	let mut result = Vec::new();

	for line in read_to_string(filename).unwrap().lines() {
		result.push(line.to_string());
	}

	result
}

fn main() {

	let mut all = Vec::new();

	/*#[derive(Debug)]
	struct User {
		name: String,
		age: u32,
		state: String
	}*/

	let meep = readLines("./data.txt");
	for i in meep {
		if i.contains("name") {
		} else {
			let parts: Vec<&str> = i.split(", ").collect();
			let cName = parts.get(0).unwrap().to_string();
			let iAge: u32 = parts.get(1).expect("reason").parse().expect("Not a number!");
			let cState = parts.get(2).unwrap().to_string();

			let usertup = (cName.clone(), iAge, cState.clone());
			all.push(usertup);

			/*let mut usr = User {
				name: cName.clone(),
				age: iAge,
				state: cState.clone()
			};
			all.push(usr);*/

			//println!("Name: {cName}\nAge: {iAge}\nState: {cState}");
		}
	}

	// println!("{:?}", all);

	for i in all {
		let (mut name, mut age, mut state) = i;
		println!("Name: {name}\nAge: {age}\nState: {state}");
	}

}