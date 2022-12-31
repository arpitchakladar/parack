use std::fs::File;
use std::io::{
	BufRead,
	BufReader
};

use crate::hash::HashFunction;

pub fn wordlist_search(hash: HashFunction, wordlist_file: &str, password_list_file: &str) {
	let wordlist_file = File::open(wordlist_file).unwrap();
	let wordlist_file_reader = BufReader::new(wordlist_file);

	let password_list: Vec<String> = BufReader::new(File::open(password_list_file).unwrap())
		.lines()
		.map(|password| password.unwrap())
		.collect();

	for line in wordlist_file_reader.lines() {
		if let Ok(line) = line {
			for password in &password_list {
				let splitted_password: Vec<&str> = password.split(":").collect();
				let password = splitted_password[0].trim();
				let p = line.trim();
				let pass = {
					if splitted_password.len() > 1 {
						p.to_string() + splitted_password[1].trim()
					} else {
						p.to_string()
					}
				};
				if hash(&pass).eq_ignore_ascii_case(password) {
					println!("Found password \x1b[1;32m{}\x1b[m", p);
					break;
				}
			}
		}
	}
}
