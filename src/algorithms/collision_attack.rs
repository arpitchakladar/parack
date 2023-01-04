use std::io::{
	BufRead,
	BufReader
};
use std::collections::HashMap;

use crate::hash::HashFunction;
use crate::utils::open_file;

fn create_repeating_string_buffer(length: usize) -> Vec<u8> {
	let mut buffer = Vec::with_capacity(length);
	for _ in 0..length {
		buffer.push('!' as u8);
	}
	buffer
}

pub fn collision_attack(hash: HashFunction, password_list_file_path: &str) -> Result<HashMap<String, String>, &'static str> {
	let password_list_file_reader = BufReader::new(open_file(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)?);

	let mut passwords = HashMap::new();
	for password in password_list_file_reader.lines() {
		if let Ok(password) = password {
			let splitted_password: Vec<&str> = password.split(":").collect();
			let password = splitted_password[0].trim();
			let salt = {
				if splitted_password.len() > 1 {
					splitted_password[1].trim()
				} else {
					""
				}
			};

			let mut current_password = vec!['!' as u8];

			while !hash(
				unsafe {
					std::str::from_utf8_unchecked(&current_password)
				},
				salt
			).eq_ignore_ascii_case(password) {
				let last_index = current_password.len() - 1;
				if current_password[last_index] != '~' as u8 {
					current_password[last_index] += 1;
				}

				for i in (0..current_password.len()).rev() {
					if current_password[i] == '~' as u8 {
						if i == 0 {
							current_password = create_repeating_string_buffer(current_password.len() + 1);
							break;
						} else {
							let n = i - 1;
							if current_password[n] != '~' as u8 {
								current_password[n] += 1;
							}
							current_password[i] = '!' as u8;
						}
					} else {
						break;
					}
				}
			}

			let current_password = String::from_utf8(current_password).unwrap();
			passwords.insert(password.to_owned(), current_password);
		}
	}

	Ok(passwords)
}
