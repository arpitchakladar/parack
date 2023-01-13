use std::io::{
	BufRead,
	BufReader
};
use std::str;
use std::collections::HashMap;

use crate::utils::Resolve;
use crate::hash::HashFunction;
use crate::utils::open_file;

fn create_repeating_string_buffer(length: usize) -> Vec<u8> {
	let mut buffer = Vec::with_capacity(length);
	for _ in 0..length {
		buffer.push('!' as u8);
	}
	buffer
}

pub fn birthday_attack(hash: HashFunction, password_list_file_path: &str) -> Result<HashMap<String, String>, &'static str> {
	let password_list_file_reader = BufReader::new(open_file(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)?);

	let mut passwords = HashMap::new();
	for password in password_list_file_reader.lines() {
		if let Ok(original_password) = password {
			let splitted_password: Vec<&str> = original_password.split(":").collect();
			let password_string = splitted_password[0].trim();
			let password = hex::decode(password_string)
				.resolve("Hash in password list file is not valid hexadecimal.")?;
			let salt = {
				if splitted_password.len() > 1 {
					splitted_password[1].trim()
				} else {
					""
				}
			}.as_bytes();

			let mut current_password = vec!['!' as u8];
			while !hash(
				&current_password,
				salt
			).eq(&password) {
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

			passwords.insert(
				original_password,
				unsafe {
					String::from_utf8_unchecked(current_password)
				}
			);
		}
	}

	Ok(passwords)
}
