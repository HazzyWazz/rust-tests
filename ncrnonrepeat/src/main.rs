use std::collections::HashSet;

fn main() {
	// declare list of names
	let names = ["A", "B", "C", "D"];
	// 01 02 03 12 23 13 (nCr(4, 2))

	let names2 = names.iter();

	let mut setvec: Vec<HashSet<&str>> = Vec::new();

	// iterate over each name twice because we're checking name and name which includes two names
	for i in names2 {
		for j in names {
			if *i != j { // skip if the names are the same because we don't want A and A
				let mut a: HashSet<&str> = HashSet::new();
				a.insert(i);
				a.insert(j);
				if setvec.contains(&a) { // skip if the names have already been seen because we don't want repeats
					continue;
				} else {
					setvec.push(a);
				}
			}
		}
	}

	// print the name combination
	println!("The combinations are:");
	for i in setvec {
		i.iter();
		println!("{:?}", i);
	}
}
