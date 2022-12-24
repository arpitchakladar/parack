use std::option::Option;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};

pub fn wordlist_search<F>(hash: F, wordlist_file: &str, password: &str) -> Option<String>
	where
		F: FnOnce(&str) -> String + Copy
{
	let file = File::open(wordlist_file).unwrap();
	let reader = BufReader::new(file);

	let mut result: Option<String> = None;

	for l in reader.lines() {
		match l {
			Ok(ref line) => {
				let checked_password = line.trim().to_string();
				let hashed_password = hash(&checked_password);
				if hashed_password.eq_ignore_ascii_case(password) {
					result = Some(checked_password);
					break;
				}
			},
			Err(ref error) => println!("{}", error)
		};
	}

	result
}
