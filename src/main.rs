pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::env;
use std::time::{Instant};
use std::collections::HashMap;

use utils::{
	parse_args
};
use hash::{
	md2,
	md4,
	md5,
	sha1,
	sha256,
	sha512,
	HashFunction
};
use algorithms::{
	targeted_guess,
	wordlist_search
};

fn run_algorithm() -> Result<HashMap<String, String>, &'static str> {
	let args = env::args().collect::<Vec<String>>();

	let hash = {
		let hash_function = &args[1];

		macro_rules! get_hash_function {
			($hash:tt) => {
				if hash_function.eq_ignore_ascii_case(stringify!($hash)) {
					Ok($hash as HashFunction)
				} else {
					Err("Hash function not found.")
				}
			};

			($hash:tt, $($remaining_hash:tt),*) => {
				if hash_function.eq_ignore_ascii_case(stringify!($hash)) {
					Ok($hash as HashFunction)
				} else {
					get_hash_function!($($remaining_hash),*)
				}
			};
		}

		get_hash_function![
			md2,
			md4,
			md5,
			sha1,
			sha256,
			sha512
		]
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
			args.get("-ti").unwrap(),
			args.get("-pl").unwrap()
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
			args.get("-wl").unwrap(),
			args.get("-pl").unwrap()
		)
	} else {
		Err("Algorithm not found.")
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
