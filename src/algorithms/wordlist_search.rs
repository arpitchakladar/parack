use std::io::{
	BufRead,
	BufReader
};
use std::collections::HashMap;

use crate::hash::HashFunction;
use crate::utils::open_file;

pub fn wordlist_search(hash: HashFunction, wordlist_file_path: &str, password_list_file_path: &str) -> Result<HashMap<String, String>, &'static str> {
	let mut password_list = BufReader::new(open_file(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)?).lines()
		.map(|password| {
			let password = password.unwrap();
			let splitted_password = password.split(":").collect::<Vec<&str>>();
			let password = splitted_password[0].trim().to_owned();
			let salt = if splitted_password.len() > 1 {
				splitted_password[1].trim()
			} else {
				""
			}.to_owned();
			(password, salt, true)
		})
		.collect::<Vec<(String, String, bool)>>();

	let wordlist_file_reader = BufReader::new(open_file(
		wordlist_file_path,
		"Word list file not found.",
		"Failed to open word list file."
	)?);

	let mut passwords = HashMap::new();
	for line in wordlist_file_reader.lines() {
		if let Ok(checked_password) = line {
			let mut no_passwords_left = true;

			for (password, salt, uncracked) in &mut password_list {
				if *uncracked {
					no_passwords_left = false;
					let hashed_password = hash(&checked_password, salt);
					if hashed_password.eq_ignore_ascii_case(password) {
						passwords.insert(password.clone(), checked_password);
						*uncracked = false;
						break;
					}
				}
			}

			if no_passwords_left {
				break;
			}
		}
	}

	Ok(passwords)
}
