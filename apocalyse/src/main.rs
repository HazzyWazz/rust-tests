use num_bigint::{BigInt, Sign};

fn main() {
	// Apocalyptic number: contains 666
	// Apocalypse number: is 666 long
	// println!("{}", u128::MAX);
	let z: Vec<u32> = vec![0];
	let t: Vec<u32> = vec![2];
	let stopperV: Vec<u32> = vec![65536];
	let stopper = BigInt::new(Sign::Plus, stopperV);
	let lastP: Vec<u32> = vec![29784];
	let last = BigInt::new(Sign::Plus, lastP);
	let mut pastLastCounter = 0;
	let mut power = BigInt::new(Sign::Plus, z);
	let two = BigInt::new(Sign::Plus, t);
	loop {
		if power > stopper {
			break;
		}
		let number = two.pow(power.clone().try_into().unwrap()).to_string();
		//let apocalyptic: u128 = number.matches("666").collect::<Vec<_>>().len().try_into().unwrap();
		let apocalyptic = number.matches("666").count();
		if apocalyptic > 0 {
			println!("2^{}", power);
			println!("Occurrences: {}", apocalyptic);
			if number.len() == 666 {
				println!("Apocalypse number")
			}
		println!("--------------")
		} else if power > last {
			println!("Found non apocalyptic past 2^29784: 2^{}", power);
			break;
		}
	power += 1;
	}
}
