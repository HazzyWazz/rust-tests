// port of "ism2.py"

// print highest score
// print average score
// print name of person who achieved highest score
// do the same, but use a function(list) to get the person and score
// print names of people with scores higher than X (30)
// search people in data set

fn maxindex(list: &[u64]) -> String {
	let m = list.iter().max().expect("Error while trying to get highest value!");
	let i_of_high = list.iter().position(|&x| x.to_string() == m.to_string()).unwrap();
	i_of_high.to_string()
}

fn main() {
	let scores = [45, 32, 28, 30, 13, 25, 17, 36, 40, 25];
	let people = ["Agyalá Gyula", "Békés Csaba", "Cicam Ica", "Futó Rózsa", "Kó Laci", "Meta Flóra", "Papl Andi", "Ultra Viola", "Vizi Lóránt", "Zok Niki"];

	// 1. print highest score (45)
	let max = scores.iter().max().expect("Error while trying to get highest value!");
	println!("Highest score: {}", max);

	// 2. print average (29.1)
	let length: u64 = scores.len().try_into().unwrap();

	let total: u64 = scores.iter().sum();
	let average = total as f64 / length as f64;
	println!("Average score: {}", average);

	// 3. get name of person with highest score (Agyalá Gyula: 45)
	let index_of_highest = scores.iter().position(|&x| x == *max).unwrap();
	println!("Highest score {} achieved by {}", scores[index_of_highest], people[index_of_highest]);

	// 4. same with func
	let msc: usize = maxindex(&scores).parse().expect("Error parsing number from index!");
	println!("Highest score {} achieved by {}", scores[msc], people[msc]);	

	// 5. print names of people with scores higher than X (30)
	// for score in scores {
	// 	if score > 30 {
	// 		let scoreindex = scores.iter().position(|&x| x == score).unwrap();
	// 		println!("{} got a score of {} (over 30)", people[scoreindex], scores[scoreindex]);
	// 	}
	// }
	scores
		.into_iter()
		.filter(|&score| score > 30)
		.for_each(|score| {
			let scoreindex = scores.iter().position(|&x| x == score).unwrap();
			println!("{} got a score of {} (over 30)", people[scoreindex], scores[scoreindex]);
		});
}