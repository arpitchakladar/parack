use std::io::{
	BufRead,
	BufReader
};
use std::collections::HashMap;
use std::str;

use crate::utils::Resolve;
use crate::hash::HashFunction;
use crate::utils::open_file;

pub fn wordlist_search(hash: HashFunction, wordlist_file_path: &str, password_list_file_path: &str) -> Result<HashMap<String, String>, &'static str> {
	let password_list_reader = BufReader::new(open_file(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)?);

	let mut password_list = Vec::new();
	for line in password_list_reader.lines() {
		if let Ok(password) = line {
			let splitted_password = password.split(":").collect::<Vec<&str>>();
			let password = hex::decode(splitted_password[0].trim())
				.resolve("Hash in password list file is not valid hexadecimal.")?;
			let salt = if splitted_password.len() > 1 {
				splitted_password[1].trim()
			} else {
				""
			}.to_owned().into_bytes();
			password_list.push((password, salt, true))
		}
	}

	let wordlist_file_reader = BufReader::new(open_file(
		wordlist_file_path,
		"Word list file not found.",
		"Failed to open word list file."
	)?);

	let mut count = 0usize;
	let mut passwords = HashMap::new();
	'outer: for line in wordlist_file_reader.lines() {
		if let Ok(checked_password) = line {
			for (password, salt, uncracked) in &mut password_list {
				if *uncracked {
					let hashed_password = hash(checked_password.as_bytes(), salt);
					if hashed_password.eq(password) {
						passwords.insert(
							hex::encode(password)
								+ ":"
								+ unsafe {
									str::from_utf8_unchecked(salt)
								},
							checked_password
						);
						*uncracked = false;
						count += 1;
						if count >= password_list.len() {
							break 'outer;
						} else {
							continue 'outer;
						}
					}
				}
			}
		}
	}

	Ok(passwords)
}
