pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::env;
use std::time::{Instant};
use std::collections::HashMap;

use hash::{
	md5,
	sha256,
	sha512,
	HashFunction
};
use algorithms::{
	targeted_guess,
	wordlist_search
};

fn parse_args<'a>(args: &'a Vec<String>, fields: Vec<(&'a str, &'a str)>) -> Result<HashMap<&'a str, &'a str>, String> {
	let mut res: HashMap<&str, &str> = HashMap::with_capacity(fields.len());
	for i in 0..args.len() {
		for (command_line_arg, _) in &fields {
			if args[i].trim().eq_ignore_ascii_case(command_line_arg) {
				res.insert(command_line_arg, &args[i + 1]);
			}
		}
	}

	for (command_line_arg, error_message) in fields {
		if !res.contains_key(command_line_arg) {
			return Err(error_message.to_owned());
		}
	}

	Ok(res)
}

fn expand_path(path: &str) -> Result<String, String> {
	if path.starts_with("~/") {
		match env::var("HOME") {
			Ok(home) => Ok(home + &path[2..]),
			Err(_) => Err(format!("Failed to resolve path \"{}\".", path))
		}
	} else {
		Ok(path.to_owned())
	}
}

fn run_algorithm() -> Result<Vec<String>, String> {
	let args = env::args().collect::<Vec<String>>();

	let hash = {
		let hash_function = &args[1];
		if hash_function.eq_ignore_ascii_case("md5") {
			Ok(md5 as HashFunction)
		} else if hash_function.eq_ignore_ascii_case("sha256") {
			Ok(sha256 as HashFunction)
		} else if hash_function.eq_ignore_ascii_case("sha512") {
			Ok(sha512 as HashFunction)
		} else {
			Err(format!("Hash function \"{}\" not found.", hash_function))
		}
	}?;

	let algorithm = &args[2];
	if algorithm.eq_ignore_ascii_case("targeted_guess") {
		let args = parse_args(
			&args,
			vec![
				("-ti", "Target information file not provided."),
				("-pl", "Password list file not provided.")
			]
		)?;

		targeted_guess(
			hash,
			&expand_path(args.get("-ti").unwrap())?,
			&expand_path(args.get("-pl").unwrap())?
		)
	} else if algorithm.eq_ignore_ascii_case("wordlist_search") {
		let args = parse_args(
			&args,
			vec![
				("-wl", "Word list file not provided."),
				("-pl", "Password list file not provided.")
			]
		)?;

		wordlist_search(
			hash,
			&expand_path(args.get("-wl").unwrap())?,
			&expand_path(args.get("-pl").unwrap())?
		)
	} else {
		Err(format!("Algorithm \"{}\" not found.", algorithm))
	}
}

fn main() {
	let current = Instant::now();

	match run_algorithm() {
		Ok(passwords) => println!("Found passwords {:?}", passwords),
		Err(message) => println!("\x1b[0;31m{}\x1b[m", message)
	}

	println!("Time elapsed is: {:?}", current.elapsed());
}
