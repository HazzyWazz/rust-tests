// 99 bottles


fn main() {
	for num in (1..100).rev() {
		println!("{num} bottles of beer on the wall, {num} bottles of beer,");
		println!("Take one down, pass it around, {} bottles of beer on the wall", num-1)
	}
}