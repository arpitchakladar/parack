use std::fs::File;
use std::io;
use std::io::{
	BufRead,
	BufReader
};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use serde_yaml;

use crate::commonly_used;
use crate::hash::HashFunction;
use crate::utils::rc_ref_cell;
use crate::combinations::{
	Combinations,
	ArrayCombinations,
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

fn generate_password_patterns(target_information_file_path: &str) -> Result<Vec<Box<dyn Combinations>>, String> {
	let target_information_file;
	match File::open(target_information_file_path) {
		Ok(file) => {
			target_information_file = file;
		},
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				return Err("Target information file not found.".to_owned());
			},
			_ => {
				return Err("Failed to open target information file.".to_owned());
			}
		}
	}
	let target_information: BTreeMap<String, Vec<String>>;
	if let Ok(data) = serde_yaml::from_reader(target_information_file) {
		target_information = data;
	} else {
		return Err("Failed to parse target information file.".to_owned());
	}
	let names;
	if let Some(data) = target_information.get("names") {
		names = Rc::new(data.clone());
	} else {
		return Err("Names not provided in target information file.".to_owned());
	}
	let numbers;
	if let Some(data) = target_information.get("numbers") {
		numbers = Rc::new(data.clone());
	} else {
		return Err("Numbers not provided in target information file.".to_owned());
	}
	let symbols = Rc::new(commonly_used::symbols());
	let common_texts = Rc::new(commonly_used::texts());
	let common_numbers = Rc::new(commonly_used::numbers());

	if let Some(patterns) = target_information.get("patterns") {
		let mut pattern_combinations: Vec<Box<dyn Combinations>> = Vec::new();

		for pattern in patterns {
			let pattern = pattern.trim();
			let combinations: Box<dyn Combinations>;
			if pattern.len() > 2 {
				let get_combination = |i| -> Result<Rc<RefCell<dyn Combinations>>, String> {
					match pattern.chars().nth(i) {
						Some('n') => Ok(rc_ref_cell!(NameCombinations::new(names.clone()))),
						Some('0') => Ok(rc_ref_cell!(SequenceCombinations::new(numbers.clone()))),
						Some('$') => Ok(rc_ref_cell!(ArrayCombinations::new(symbols.clone()))),
						Some('x') => Ok(rc_ref_cell!(SequenceCombinations::new(common_texts.clone()))),
						Some('y') => Ok(rc_ref_cell!(SequenceCombinations::new(common_numbers.clone()))),
						_ => Err("Failed to parse target information file.".to_owned())
					}
				};
				let mut combine_combinations: CombineCombinations = CombineCombinations::new(
					get_combination(pattern.len() - 3)?,
					get_combination(pattern.len() - 1)?
				);
				if pattern.len() > 4 {
					let mut i = pattern.len() - 5;
					loop {
						combine_combinations = CombineCombinations::new(
							get_combination(i)?,
							rc_ref_cell!(combine_combinations)
						);
						if i < 2 {
							break;
						} else {
							i -= 2;
						}
					}
				}
				combinations = Box::new(combine_combinations);
			} else {
				combinations = match pattern.chars().nth(0) {
					Some('n') => Box::new(NameCombinations::new(names.clone())),
					Some('0') => Box::new(SequenceCombinations::new(numbers.clone())),
					Some('$') => Box::new(ArrayCombinations::new(symbols.clone())),
					Some('x') => Box::new(SequenceCombinations::new(common_texts.clone())),
					Some('y') => Box::new(SequenceCombinations::new(common_numbers.clone())),
					_ => return Err("Failed to parse target information file.".to_owned())
				};
			}
			pattern_combinations.push(combinations);
		}

		Ok(pattern_combinations)
	} else {
		Err("Patterns not provided in target information file.".to_owned())
	}
}

pub fn targeted_guess(hash: HashFunction, target_information_file_path: &str, password_list_file_path: &str) -> Result<Vec<String>, String> {
	let mut patterns = generate_password_patterns(target_information_file_path)?;

	let password_list_file_reader;
	match File::open(password_list_file_path) {
		Ok(password_list_file) => {
			password_list_file_reader = BufReader::new(password_list_file);
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

	let mut passwords = Vec::new();
	for password in password_list_file_reader.lines() {
		if let Ok(password) = password {
			let splitted_password: Vec<&str> = password.split(":").collect();
			let password = splitted_password[0].trim();
			let salt = {
				if splitted_password.len() > 1 {
					Some(splitted_password[1].trim())
				} else {
					None
				}
			};

			for combinations in &mut patterns {
				let mut done = false;

				combinations.reset();
				for current_password in combinations {
					let hashed_password = hash(&{
						if let Some(salt) = salt {
							current_password.clone() + salt
						} else {
							current_password.clone()
						}
					});

					if hashed_password.eq_ignore_ascii_case(password) {
						passwords.push(current_password);
						done = true;
						break;
					}
				}

				if done {
					break;
				}
			}
		}
	}

	Ok(passwords)
}
