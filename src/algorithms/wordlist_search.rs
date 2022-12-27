use std::option::Option;
use std::fs::File;
use std::io::{
	BufRead,
	BufReader
};

use crate::hash::HashFunction;

pub fn wordlist_search(hash: HashFunction, wordlist_file: &str, password: &str) -> Option<String> {
	let file = File::open(wordlist_file).unwrap();
	let reader = BufReader::new(file);

	for l in reader.lines() {
		if let Ok(line) = l {
			let checked_password = line.trim().to_string();
			let hashed_password = hash(&checked_password);
			if hashed_password.eq_ignore_ascii_case(password) {
				return Some(checked_password);
			}
		}
	}

	None
}
