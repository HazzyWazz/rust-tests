#![allow(non_snake_case)]

fn fnFib(n: i32) -> i32 {
	if n <= 1 {
		1
	} else {
		fnFib(n-2) + fnFib(n-1)
	}
}

fn fnFin2(n: i32) -> i32 {
	0
}

fn main() {
	// assert_eq!(fnFib(23), 13);
	println!("{}", fnFib(48));
}