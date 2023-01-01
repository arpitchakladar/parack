use std::fs::File;
use std::io;
use std::io::{
	BufRead,
	BufReader
};

use crate::hash::HashFunction;

pub fn wordlist_search(hash: HashFunction, wordlist_file_path: &str, password_list_file_path: &str) -> Result<Vec<String>, String> {
	let password_list;
	match File::open(password_list_file_path) {
		Ok(password_list_file) => {
			password_list = BufReader::new(password_list_file)
				.lines()
				.map(|password| password.unwrap())
				.collect::<Vec<String>>();
		},
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				return Err("Password list file not found.".to_owned());
			},
			_ => {
				return Err("Failed to open password list file.".to_owned());
			}
		}
	}

	let wordlist_file_reader;
	match File::open(wordlist_file_path) {
		Ok(wordlist_file) => {
			wordlist_file_reader = BufReader::new(wordlist_file);
		},
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				return Err("Word list file not found.".to_owned());
			},
			_ => {
				return Err("Failed to open word list file.".to_owned());
			}
		}
	}

	let mut passwords = Vec::new();

	for line in wordlist_file_reader.lines() {
		if let Ok(checked_password) = line {
			for password in &password_list {
				let splitted_password: Vec<&str> = password.split(":").collect();
				let password = splitted_password[0].trim();
				let hashed_password = hash(&{
					if splitted_password.len() > 1 {
						checked_password.to_owned() + splitted_password[1].trim()
					} else {
						checked_password.to_owned()
					}
				});
				if hashed_password.eq_ignore_ascii_case(password) {
					passwords.push(checked_password.to_owned());
					break;
				}
			}
		}
	}

	Ok(passwords)
}
