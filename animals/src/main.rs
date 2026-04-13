#![allow(non_snake_case)]
struct Cow {
	name: &'static str,
	pattern: &'static str,
	spayed: bool
}

struct Cat {
	name: &'static str,
	pattern: &'static str,
	spayed: bool
}

struct Dog {
	name: &'static str,
	pattern: &'static str,
	spayed: bool
}

trait Animal {
	fn new(name: &'static str, pattern: &'static str, spayed: bool) -> Self;
	fn name(&self) -> &'static str;
	fn noise(&self) -> &'static str;
	fn talk(&self) {
		println!("{} says {}", self.name(), self.noise())
	}
}

impl Animal for Cow {
	fn new(name: &'static str, pattern: &'static str, spayed: bool) -> Cow {
		Cow {name: name, pattern: pattern, spayed}
	}

	fn name(&self) -> &'static str {
		self.name
	}

	fn noise(&self) -> &'static str {
		if self.spayed {
			"angry moo"
		} else {
			"moo"
		}
	}
}

impl Animal for Cat {
	fn new(name: &'static str, pattern: &'static str, spayed: bool) -> Cat {
		Cat {name: name, pattern: pattern, spayed: spayed}
	}

	fn name(&self) -> &'static str {
		self.name
	}

	fn noise(&self) -> &'static str {
		if self.spayed {
			"meow"
		} else {
			"mrow"
		}
	}
}

impl Animal for Dog {
	fn new(name: &'static str, pattern: &'static str, spayed: bool) -> Dog {
		Dog {name: name, pattern: pattern, spayed: spayed}
	}

	fn name(&self) -> &'static str {
		self.name
	}

	fn noise(&self) -> &'static str {
		if self.spayed {
			"woof"
		} else {
			"arf"
		}
	}
}

impl ToString for Cow {
	fn to_string(&self) -> String {
		format!("{} the {} cow", &self.name, &self.pattern)
	}
}

impl ToString for Cat {
	fn to_string(&self) -> String {
		format!("{} the {} cat", &self.name, &self.pattern)
	}
}

impl ToString for Dog {
	fn to_string(&self) -> String {
		format!("{} the {} dog", &self.name, &self.pattern)
	}
}

fn main() {
	let milka = Cow::new("Milka", "Purple spotted", false);
	// println!("{} looks {}", milka.name, milka.pattern);
	println!("{}", milka.to_string());
	milka.talk();

	let chili = Dog::new("Chili", "Plain black", false);
	// println!("{} looks {}", chili.name, chili.pattern);
	println!("{}", chili.to_string());
	chili.talk();

	let atomicToaster = Cat::new("Atomic Toaster", "Calico", true);
	// println!("{} looks {}", atomicToaster.name, atomicToaster.pattern);
	println!("{}", atomicToaster.to_string());
	atomicToaster.talk();
}
