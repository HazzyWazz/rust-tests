// python equivalent of lists
// let's try structs and
// let's parse some CSV type data

// I left in the struct for educational purposes

use std::fs::read_to_string;  // import read to string


fn readLines(filename: &str) -> Vec<String> {  // readlines functions with string parameter (the file name) will return a vector
	let mut result = Vec::new();  // create new empty mutable vector

	for line in read_to_string(filename).unwrap().lines() { // for each line in the file
		result.push(line.to_string()); //push the line into the empty vector
	}

	result //return the vector
}

fn main() {

	let mut all = Vec::new(); // create new empty mut vector

	/*#[derive(Debug)]
	struct User {
		name: String,
		age: u32,
		state: String
	}*/

	let meep = readLines("./data.txt");  // read file into meep
	for i in meep { // for every vector in meep
		if i.contains("name") { // skip first line (CSV)
		} else {
			let parts: Vec<&str> = i.split(", ").collect(); // let each column value be a vector item
			let cName = parts.get(0).unwrap().to_string(); // let name, age and state values be string, int and string
			let iAge: u32 = parts.get(1).expect("reason").parse().expect("Not a number!"); // parsing
			let cState = parts.get(2).unwrap().to_string();

			// create tuple with String, u32, String values
			let usertup = (cName.clone(), iAge, cState.clone());
			// push tuple into all vector
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

	for i in all { // for each tuple in all
		let (mut name, mut age, mut state) = i;  //create new tuples of i (to read)
		println!("Name: {name}\nAge: {age}\nState: {state}"); // print
	}

}