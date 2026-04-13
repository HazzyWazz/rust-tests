#![allow(dead_code)]
#![allow(non_snake_case)]
use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

fn readInput() -> String {
	let mut userInput = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut userInput);
	userInput
}


fn main() -> std::io::Result<()> {
	println!("Please input your host limit (default: 255): ");
	let mut limit: i32 = readInput().trim().parse().unwrap();

	let file = File::create("./config.csv")?;
	let mut file = LineWriter::new(file);

	if limit != 0 {
		if limit == 0 {
			println!("Invalid limit (0), using default value");
			limit = 255;
		}
		for network in 0..256 {
			for host in 0..limit+1 {
				match host {
					// 0 => println!("Helyszín;172.16.{}.{};;", network, host),
					0 => {
						file.write_all(format!("Helyszín;172.16.{}.{};;", network, host).as_bytes())?;
						file.write_all(b"\n")?;
					},
					// _ => println!("Számítógép;172.16.{}.{};172.16.{}.{}", network, host, network, host)
					_ => {
						file.write_all(format!("Számítógép;172.16.{}.{};172.16.{}.{};", network, host, network, host).as_bytes())?;
						file.write_all(b"\n")?;
					}
				}
			}
		}
	} else {
		for network in 0..256 {
			for host in 0..255 {
				match host {
					// 0 => println!("Helyszín;172.16.{}.{};;", network, host),
					0 => {
						file.write_all(format!("Helyszín;172.16.{}.{};;", network, host).as_bytes())?;
						file.write_all(b"\n")?;
					}
					// _ => println!("Számítógép;172.16.{}.{};172.16.{}.{}", network, host, network, host)
					_ => {
						file.write_all(format!("Számítógép;172.16.{}.{};172.16.{}.{};", network, host, network, host).as_bytes())?;
						file.write_all(b"\n")?;
					}
				}
			}
		}
	}
	Ok(())

}
