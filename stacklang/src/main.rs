#![allow(non_snake_case)]
#![allow(unused_variables)]

use core::panic;


#[derive(Debug)]
enum Type<'a> {
	Number(f64),
	Named(&'a str)
}

/*enum Instruction {
	NUM = 0,
	ADD = 1,
	SUB = 2,
	MUL = 3,
	OUT = 125,
	DBG = 126,
	END = 127
}*/

fn parse(input: &str) -> Vec<Type> {
	let wholeVec = input.split(" ").collect::<Vec<&str>>();
	let mut newVec: Vec<Type> = vec!();
	for elem in wholeVec {
		match elem.parse::<f64>() {
			Ok(elem) => newVec.push(Type::Number(elem)),
			Err(_) => newVec.push(Type::Named(elem)),
		}
	}
	println!("{:?}", newVec);
	newVec
}

fn evaluate(input: Vec<Type>) -> f64 {
	let mut stack: Vec<f64> = Vec::new();
	let mut output: f64 = 0.0;
	for elem in input {
		match elem {
			Type::Number(value) => stack.push(value),
			Type::Named(inst) => match inst {
				"+" => {
					let x = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					let out = x + y;
					stack.push(out);
				},
				"-" => {
					let x = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					let out = x - y;
					stack.push(out);
				},
				"*" => {
					let x = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					let out = x * y;
					stack.push(out);
				},
				"/" => {
					let x = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					let out = x / y;
					stack.push(out);
				},
				"print" => {
					output = stack.pop().unwrap();
				},
				_ => panic!("How")
			} 
		}
	}
	output
}

fn main() {
	// let instruction = "1 2 + print";
	let instruction = "2 3 4 * + print";
	let parsedInst = parse(instruction);
	let evalInst = evaluate(parsedInst);
	println!("{}", evalInst)
}
