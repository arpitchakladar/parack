pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::env;
use std::time::{Instant};

use utils::{
	parse_args,
	expand_path
};
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
