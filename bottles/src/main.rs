// 99 bottles


fn main() {
	for num in (1..100).rev() {
		if num != 1 {
			println!("{num} bottles of beer on the wall, {num} bottles of beer,");
			if num == 2 {
				println!("Take one down, pass it around, {} bottle of beer on the wall", num-1);
			} else {
				println!("Take one down, pass it around, {} bottles of beer on the wall", num-1);
			}
		}
		else {
			println!("{num} bottle of beer on the wall, {num} bottle of beer,");
			println!("Take one down, pass it around, no more bottles of beer on the wall");
		}
	}
}