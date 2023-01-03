use std::fs::File;
use std::io;
use std::io::{
	BufRead,
	BufReader
};

use crate::hash::HashFunction;
use crate::utils::try_open_file;

pub fn wordlist_search(hash: HashFunction, wordlist_file_path: &str, password_list_file_path: &str) -> Result<Vec<String>, String> {
	let password_list = BufReader::new(try_open_file!(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)).lines()
		.map(|password| {
			let password = password.unwrap();
			let splitted_password = password.split(":").collect::<Vec<&str>>();
			let password = splitted_password[0].trim().to_owned();
			let salt = if splitted_password.len() > 1 {
				splitted_password[1].trim()
			} else {
				""
			}.to_owned();
			(password, salt)
		})
		.collect::<Vec<(String, String)>>();

	let wordlist_file_reader = BufReader::new(try_open_file!(
		wordlist_file_path,
		"Word list file not found.",
		"Failed to open word list file."
	));

	let mut passwords = Vec::new();
	for line in wordlist_file_reader.lines() {
		if let Ok(checked_password) = line {
			for (password, salt) in &password_list {
				let hashed_password = hash(&checked_password, salt);
				if hashed_password.eq_ignore_ascii_case(password) {
					passwords.push(checked_password);
					break;
				}
			}
		}
	}

	Ok(passwords)
}
