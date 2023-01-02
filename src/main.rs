pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::env;
use std::time::{Instant};

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

fn expand_path(path: &str) -> String {
	if path.starts_with("~/") {
		return env::var("HOME").unwrap() + &path[2..]
	} else {
		path.to_owned()
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
		let mut target_information_file_path = None;
		let mut password_list_file_path = None;
		for i in 3..args.len() {
			let arg = args[i].trim();
			if arg.eq_ignore_ascii_case("-ti") {
				target_information_file_path = Some(&args[i + 1]);
			} else if arg.eq_ignore_ascii_case("-pl") {
				password_list_file_path = Some(&args[i + 1]);
			}
		}

		if target_information_file_path.is_none() {
			Err("Target information file not provided.".to_owned())
		} else if password_list_file_path.is_none() {
			Err("Password list file not provided.".to_owned())
		} else {
			targeted_guess(hash, &expand_path(target_information_file_path.unwrap()), &expand_path(password_list_file_path.unwrap()))
		}
	} else if algorithm.eq_ignore_ascii_case("wordlist_search") {
		let mut wordlist_file_path = None;
		let mut password_list_file_path = None;
		for i in 3..args.len() {
			let arg = args[i].trim();
			if arg.eq_ignore_ascii_case("-wl") {
				wordlist_file_path = Some(&args[i + 1]);
			} else if arg.eq_ignore_ascii_case("-pl") {
				password_list_file_path = Some(&args[i + 1]);
			}
		}

		if let None = wordlist_file_path {
			Err("Word list file not provided.".to_owned())
		} else if let None = password_list_file_path {
			Err("Password list file not provided.".to_owned())
		} else {
			wordlist_search(hash, &expand_path(wordlist_file_path.unwrap()), &expand_path(password_list_file_path.unwrap()))
		}
	} else {
		Err(format!("Algorithm \"{}\" not found.", algorithm))
	}
}

fn main() {
	let current = Instant::now();

	match run_algorithm() {
		Ok(passwords) => println!("Found passwords {:?}", passwords),
		Err(message) => println!("{}", message)
	}

	println!("Time elapsed is: {:?}", current.elapsed());
}
