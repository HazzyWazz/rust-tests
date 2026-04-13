#![allow(non_snake_case)]  // really important
use std::{fs, io::{self, Write}};
use serde_json::{json, Value}; 

fn userInput() -> String {
	let mut stdInput = String::new();
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut stdInput).expect("Error reading from STDIN");  // take input from STDIN
	stdInput
}

fn getWeatherData(lat: &str, long: &str, sd: &str, ed: &str) -> Result<String, Box<dyn std::error::Error>>{
	let link = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + &lat.to_string() + "&longitude=" + &long.to_string() + "&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&start_date=" + &sd + "&end_date=" + &ed;
	let req = reqwest::blocking::get(link)?.text();
	let t = String::from(req.unwrap());
	Ok(t)
}

fn writeResult(resVec: Vec<Value>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut outputfile = fs::File::create(format!("./output/{filename}"))?;
	let resString = format!("Date: {}\nLatitude: {}\nLongitude: {}\nTmin: {}°C\nTmax: {}°C\nPrecipitation: {}mm", resVec[0].as_str().unwrap().replace('"', ""), resVec[1], resVec[2], resVec[3], resVec[4],resVec[5]);
	println!("Writing...");
	let _ = outputfile.write(resString.as_bytes());
	println!("Written!");
	Ok(())
}

fn main() {
	print!("Enter a latitude: ");
	let binding = userInput();
	let latitude: &str = binding
				.as_str()
				.trim();
	print!("Enter a longitude: ");
	let binding = userInput();
	let longitude: &str = binding
				.as_str()
				.trim();
	print!("Enter a start date: (YYYY-MM-DD) ");
	let binding = userInput();
 	let startDate: &str = binding
				.as_str()
				.trim();
	print!("Enter an end date: (YYYY-MM-DD) ");
	let binding = userInput();
 	let endDate: &str = binding
				.as_str()
				.trim();

	let weatherData = getWeatherData(latitude, longitude, startDate, endDate).unwrap();
	let parsedWD: Value = serde_json::from_str(json!(&weatherData).as_str().unwrap()).unwrap();
	print!("{}\n {}\n {}\n {}\n {}\n {}",
		parsedWD["daily"]["time"][0],
		parsedWD["latitude"],
		parsedWD["longitude"],
		parsedWD["daily"]["temperature_2m_min"][0],
		parsedWD["daily"]["temperature_2m_max"][0],
		parsedWD["daily"]["precipitation_sum"][0],
	);
	let tempVec = vec![
			parsedWD["daily"]["time"][0].clone(),
			parsedWD["latitude"].clone(),
			parsedWD["longitude"].clone(),
			parsedWD["daily"]["temperature_2m_min"][0].clone(),
			parsedWD["daily"]["temperature_2m_max"][0].clone(),
			parsedWD["daily"]["precipitation_sum"][0].clone(),
			];
	let _ = writeResult(tempVec, "./coordweather.txt");
}