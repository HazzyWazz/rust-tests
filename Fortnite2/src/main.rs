#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{fs, thread, time, io};
use rand::Rng;
use rand::seq::SliceRandom;

fn loadNames() -> Vec<String> {
	let mut usernames = Vec::new();
	let query = fs::read_to_string("./usernames.txt").expect("Error reading usernames.txt!");
		for name in query.lines() {
				if name.len() >= 5 {
					usernames.push(name.to_string());
				}
			}
	usernames
}

fn loadUsers(nameVec: Vec<String>) -> Vec<String> { // am too simple for that soooo
	let mut rng = rand::thread_rng();
	let mut users: Vec<String> = Vec::new();
	while users.len() < 100 {
		let username = nameVec.choose(&mut rng).unwrap();
		users.push(username.to_string());
		println!("[{}/100] {} has joined the game", users.len(), username);
		//thread::sleep(time::Duration::from_millis(rng.gen_range(500..1000)));
	}
	println!("All users logged in! Starting game in 10 seconds");
	thread::sleep(time::Duration::from_millis(10000));
	users
}

fn killMessage(remaining: i32, killer: String, killed: String) {
	let mut rng = rand::thread_rng();
	let mut snipeDist = rng.gen_range(1..256);
	let mut killId = rng.gen_range(1..15);
	match killId {
		1 => println!("[{}/100] {} 'sploded {}", remaining, killer, killed),
		2 => println!("[{}/100] {} NO-SCOPED {} ({} meters)", remaining, killer, killed, snipeDist),
		3 => println!("[{}/100] {} bludgeoned {}", remaining, killer, killed),
		4 => println!("[{}/100] {} shotgunned {}", remaining, killer, killed),
		5 => println!("[{}/100] {} eliminated {} with a pistol", remaining, killer, killed),
		6 => println!("[{}/100] {} eliminated {} with a rifle", remaining, killer, killed),
		7 => println!("[{}/100] {} eliminated {} with a marksman rifle", remaining, killer, killed),
		8 => println!("[{}/100] {} eliminated {} with a SMG	", remaining, killer, killed),
		9 => println!("[{}/100] {} eliminated {} with a minigun", remaining, killer, killed),
		10 => println!("[{}/100] {} eliminated {} with a LMG", remaining, killer, killed),
		11 => println!("[{}/100] {} eliminated {} with a trap", remaining, killer, killed),
		12 => println!("[{}/100] {} eliminated {} with a turret", remaining, killer, killed),
		13 => println!("[{}/100] {} eliminated {} with a bow", remaining, killer, killed),
		14 => println!("[{}/100] {} sniped {} ({} meters)", remaining, killer, killed, snipeDist),
		_ => println!("[{}/100] {} was struck by the ban hammer\n{} was struck by the ban hammer", remaining, killer, killed),
	}
}

fn suicideMessage(remaining: i32, killer: String) {
	let mut rng = rand::thread_rng();
	let mut distance = rng.gen_range(1..33);
	let mut deathId = rng.gen_range(1..19);
	match deathId {
		1 => println!("[{}/100] {} was lost in the storm", remaining, killer),
		2 => println!("[{}/100] {} was eliminated", remaining, killer),
		3 => println!("[{}/100] {} played themselves", remaining, killer),
		4 => println!("[{}/100] {} knocked themselves out", remaining, killer),
		5 => println!("[{}/100] {} was knocked out", remaining, killer),
		6 => println!("[{}/100] {} will return!", remaining, killer),
		7 => println!("[{}/100] {} bounced!", remaining, killer),
		8 => println!("[{}/100] {} noped out", remaining, killer),
		9 => println!("[{}/100] {} missed their stop", remaining, killer),
		10 => println!("[{}/100] {} bailed early", remaining, killer),
		11 => println!("[{}/100] {} is gonna nail it next time", remaining, killer),
		12 => println!("[{}/100] {} opted out", remaining, killer),
		13 => println!("[{}/100] {} took their leave", remaining, killer),
		14 => println!("[{}/100] {} checked out early", remaining, killer),
		15 => println!("[{}/100] {} is literally on fire", remaining, killer),
		16 => println!("[{}/100] {} went out with a BOOM!", remaining, killer),
		17 => println!("[{}/100] {} went out with a BANG!", remaining, killer),
		18 => println!("[{}/100] {} didn't stick the landing (Distance in meters)", remaining, killer),
		_ => println!("[{}/100] {} was struck by the ban hammer", remaining, killer),
	}
}

fn killTime(nameVec: Vec<String>) {
	let mut namesCopy = nameVec.clone(); // i see
	//let mut players// you'll hate me for this u-u
	let mut rng = rand::thread_rng();
	while namesCopy.len() != 1 {
		let mut killer = namesCopy.choose(&mut rng).unwrap();
		let mut killed = namesCopy.choose(&mut rng).unwrap();
		let mut remaining: i32 = (namesCopy.len() - 1).try_into().unwrap();
		if killer != killed {
			let killedClone = killed.clone();
			killMessage(remaining, killer.to_string(), killed.to_string());
			// namesCopy.retain(|x| &x != killed);
			namesCopy.retain(|x: &String| x != &killedClone)
		} else {
			suicideMessage(remaining, killer.to_string());
			// namesCopy.retain(|x| &x != killer);
			let killerClone = killer.clone();
			namesCopy.retain(|x: &String| x != &killerClone)
		}
		// thread::sleep(time::Duration::from_millis(rng.gen_range(3500..10000)));
		thread::sleep(time::Duration::from_millis(rng.gen_range(350..1000)));
	}
	if namesCopy.len() == 1 {
		println!("Victory Royale: {}", namesCopy[0]);
	}
}

fn main() {
	println!("Fortnite 2 server launching...");
	// thread::sleep(time::Duration::from_millis(12000));
	println!("Loading map...");
	// thread::sleep(time::Duration::from_millis(3000));
	println!("Loading assets...");
	// thread::sleep(time::Duration::from_millis(5000));
	println!("Loading done! Listening on 0.0.0.0:43553");
	//loadUsers(loadNames());
	println!("Game started!");
	killTime(loadUsers(loadNames()));
}