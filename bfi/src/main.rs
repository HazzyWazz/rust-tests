#![allow(non_snake_case)]
#![allow(unused_must_use)]
/*
>	Move the pointer to the right											iter step+1 / index of elem +1
<	Move the pointer to the left											iter step-1 / index of elem -1
+	Increment the memory cell at the pointer								elem+1
-	Decrement the memory cell at the pointer								elem-1
.	Output the character signified by the cell at the pointer				println!(elem)
,	Input a character and store it in the cell at the pointer				let elem = char
[	Jump past the matching ] if the cell at the pointer is 0				while elem == 0
]	Jump back to the matching [ if the cell at the pointer is nonzero		while elem != 0
*/

// need to keep track of which "cell" i'm at
	// i'll know because else i can1t index
// function to create vec
// mut Vec<u8> of unknown length
	// irrelevant because vecs can be unknown length; on pointer to the right if no next then create "cell"

// ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
// vec![0, 0, 72, 104, 88, 32, 8]

// +>>>>>>>++>+>+>+>++<[+[--[++>>--]->--[+[+<+[-<<+]++<<[-[->-[>>-]++<[<<]++<<-]+<<]>>>>-<<<<<++<-<<++++++[<++++++++>-]<.---<[->.[-]+++++>]>[[-]>>]]+>>--]+<+[-<+<+]++>>]<<<<[[<<]>>[-[+++<<-]+>>-]++[<<]<<<<<+>]>[->>[[>>>[>>]+[-[->>+>>>>-[-[+++<<[-]]+>>-]++[<<]]+<<]<-]<]]>>>>>>>]
// /shrug

use std::io;

fn readInput() -> String {
	println!("Please input your bf: ");
	let mut userInput = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut userInput);
	userInput
}

fn validate(inp: &str) -> bool {
	let mut valid = false;
	for char in inp.chars() {
		match char {
			'>' => valid = true,
			'<' => valid = true,
			'+' => valid = true,
			'-' => valid = true,
			'.' => valid = true,
			',' => valid = true,
			'[' => valid = true,
			']' => valid = true,
			_ => valid = false
		}
	}
	valid
}

fn interpret(bf: &str) {
	let mut memory: Vec<u8> = vec![0; 30];
	let mut pointer = 0;
	let mut value = 0;
	let mut curls = 0;
	// increment value while char is +
	// once char is not +, set index to value
	// .insert(index, elem)
	// bug: index may not even exist
		// solved by insert
	for char in bf.chars() {
		match char {
			'>' => pointer += 1,
			'<' => pointer -= 1,
			'+' => {memory[pointer] += 1;},
			'-' => {memory[pointer] -= 1;},
			'.' => println!("{}", &memory[pointer]),
			//',' => memory.insert(pointer, char.to_digit(10)),
			',' => {dbg!(&memory);},
			'[' => {curls +=1; while memory[pointer] == 0 {}; ()},
			']' => {curls -=1; while memory[pointer] != 0 {}; ()},
			_ => println!("Invalid input past validation, memory corrupted?"),
		}
		//memory.insert(pointer, value);
	}

}

fn main() {
	let binding = readInput();
	let input = binding.trim();
	println!("{}", input);
	if validate(input) {
		println!("Input is valid");
		interpret(input);
		// let a = 'A'.to_digit(10);
		// println!("{}", a.unwrap());
	} else {
		println!("Input is invalid");
	}
}