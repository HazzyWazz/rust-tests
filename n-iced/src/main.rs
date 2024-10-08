use iced::widget::{button, column, text, Column};
use magic::display;

struct Counter {
    value: i64,
}


#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
	fn update(&mut self, message: Message) {
		match message {
			Message::Increment => {self.value += 1},
			Message::Decrement => {self.value -= 1},
		}
	}

	fn view(&self) -> Column<Message> {
		let increment = button("+").on_press(Message::Increment);
		let decrement = button("-").on_press(Message::Decrement);

		let countr = text(&self.value);

		let interface = column![increment, countr, decrement];

		interface
	}
}

fn main() {
	// println!("Hello, world!");
	let mut counter = Counter { value: 0 };
	counter.update(Message::Increment);

	// Run our view logic to obtain our interface
	let interface = counter.view();
	// Display the interface to the user
	display(&interface);

	
}
