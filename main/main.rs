// first ever rust i wrote, comment below is for notes

// this is a comment
/*

numbers exist
they have bounds
math operators are the usual; + - * /

comment blocks are usually not used

not float numbers don't support decimals, important when using division

10 / 3 = 3, not 3.33333333333...4
10.0 / 3.0 = 3.333333333...4

for powers, use .pow()
2^5 would be 2.pow(5)

let x: u32 = 2  // defines x as a u32 type with a value of 2 (unsigned 32bit int)
let result = 2.pow(5)  // result = 32

2u32.pow(5) = 32 // adding the type as a suffix also works

numbers can have underscores so they're easier to read

.sqrt() only works on floats

16_f64.sqrt() = 4

*/

fn main() {
	println!("hewwo");  // macro, prints to stdout
	let name = "hazel";  // define name as string
	println!("hi {}", name);  // print "hi {name}" to stdout
	let bree = "bree";
	println!("hiii {bree}");  //add :? as suffix to debug
}