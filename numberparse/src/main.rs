use std::{io};

fn parseNum(n: char) -> i32 {
	// println!("{:?}", n);
	match n {
		'0' => 0,
		'1' => 1,
		'2' => 2,
		'3' => 3,
		'4' => 4,
		'5' => 5,
		'6' => 6,
		'7' => 7,
		'8' => 8,
		'9' => 9,
		_ => todo!()
	}
}

fn readInput() -> String {
	println!("Please input your number: ");
	let mut userInput = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut userInput);
	userInput
}

fn fconvert(ns: &str, mut nl: u32) -> i32 {
	// println!("ns: {}\nnl: {}", ns, nl);
	let mut rn: i32 = 0;
	let numiter = ns.chars();
	for c in numiter {
		rn = rn + parseNum(c)*(10_i32.pow(nl-1));
		nl = nl-1;
	}
	println!("Number: {rn}\n", );
	// dbg!(rn);
	rn
}

fn main() {
	let num = readInput();
	let snum = num.trim();
	// println!("Input is: {}", num);
	let nlen: u32 = snum.len().try_into().unwrap();
	// println!("Length is: {}", nlen);
	fconvert(&snum, nlen);
}


