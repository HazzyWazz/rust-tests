#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

use infer::Infer;
use std::io;
use std::io::prelude::*;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::path::Path;

fn extract(f: &str) {
	// specify output directory
	let output = ".\\output\\";
	// create binding to store path to cache file
	let filePathBinding = f
		.rsplit("\\")
		.collect::<Vec<_>>()
		.get(0)
		.expect("Cannot get file!")
		.split(".")
		.collect::<Vec<_>>();

	// store cache file name
	let fileName = filePathBinding.get(0).unwrap();

	// open cache file and create a buffer of 4 bytes
	let mut cacheFile = File::open(f);
	let mut fileLen = File::metadata(&File::open(f).unwrap()).unwrap().len();
	// let mut buffer = [0; fileLen];

	// read data into data length long buffer and keep it
	let mut databuffer = vec![0; fileLen.try_into().unwrap()];
	cacheFile.as_ref().expect("Cannot read file!").read_exact(&mut databuffer);
	let data = databuffer;

	// get magic of data
	let magic = data.get(0..4).unwrap();
	let kind = infer::get(&magic).expect("Filetype is known");
	// get file extension from magic
	let fileExt = kind.extension();

	// create output file
	let mut outputFileName = fileName.to_owned().to_owned() + "." + fileExt;
	let mut outputFilePath = ".\\output\\".to_owned() + &outputFileName;
	let mut outputFile = File::create(outputFilePath);
	// write data buffer to file
	outputFile.expect("Cannot write data to file!").write_all(&data);
	// alert user :3
	println!("Successfully extracted: {:?}", outputFileName);
}


fn visitDir(dir: &str) {
	// get entries in cache directory
	let entries = fs::read_dir(dir).unwrap();
	// for each entry
	for entry in entries {
		// get its path
		let path = entry.expect("Unable to get entry!").path();
		// check if it's a directory
		// if it is
		if path.is_dir() {	
			// go over that dir
			visitDir(path.to_str().unwrap());
		// if it isn't
		} else {
			// begin extraction!
			extract(path.to_str().unwrap())
;		} 
	}
}


fn main() {
	// hardcoded because i ain't gonna handle all that
	let cacheDir = "C:\\Users\\Hazzy\\Documents\\Everything\\Games\\Servers\\images";
	visitDir(cacheDir);
}
