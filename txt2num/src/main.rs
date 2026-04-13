use std::collections::HashMap;
use std::io;

fn readInput() -> String {
	let mut userInput = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut userInput);
	userInput
}

fn getNumber(txt: String) -> i64 {
	let numbers = HashMap::from([
		("zero", 0),
		("nil", 0),
		("nul", 0),
		("null", 0),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
		("ten", 10),
		("eleven", 11),
		("twelve", 12),
		("thirteen", 13),
		("fourteen", 14),
		("fifteen", 15),
		("sixteen", 16),
		("seventeen", 17),
		("eighteen", 18),
		("nineteen", 19),
		("twenty", 20),
		("thirty", 30),
		("fourty", 40),
		("fifty", 50),
		("sixty", 60),
		("seventy", 70),
		("eighty", 80),
		("ninety", 90),
	]);
	let tens = HashMap::from([
		("hundred", 100),
		("thousand", 1000),
		("million", 1000000),
		("billion", 1000000000),
	]);
	let mut txtVec = txt.trim().split(" ").collect::<Vec<&str>>();
	//dbg!(txtVec);
	// ["two", "hundred", "million",]
	txtVec.retain(|&x| x != "and");
	let txtIter = txtVec.iter();
	let mut num = 0;
	let mut temp = 0;
	for item in txtIter {
		if numbers.contains_key(item) {
			// println!("Number! {}", numbers[item]);
			temp = temp + numbers[item];
		} else if tens.contains_key(item) {
			// println!("Multiplier! {}", tens[item]);
			temp = temp*tens[item];
			if item == &"thousand" || item == &"million" || item == &"billion" {
				num = num + temp;
				temp = 0;
			}
		} else {
			panic!("Invalid input!");
		}
		//dbg!(temp);
		//dbg!(num);
	}
	num = num + temp;
	// dbg!(num);
	// one hundred twenty three million four hundred fifty six thousand seven hundred eighty nine
	// one billion two hundred thirty four million five hundred sixty seven thousand eight hundred and ninety
	num
}

fn main() {
	println!("Please input your text number: ");
	let input = readInput();
	let number = getNumber(input.clone());
	println!("Input: {}", input);
	println!("Number: {}", number);
}
