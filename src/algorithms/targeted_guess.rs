use std::fs::File;
use std::io;
use std::io::{
	BufRead,
	BufReader
};
use std::rc::Rc;
use std::collections::BTreeMap;

use serde_yaml;

use crate::commonly_used;
use crate::utils::{
	try_result,
	try_option,
	try_open_file
};
use crate::hash::HashFunction;
use crate::combinations::{
	Combinations,
	ArrayCombinations,
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

fn generate_password_patterns(target_information_file_path: &str) -> Result<Vec<Box<dyn Combinations>>, String> {
	let target_information_file = try_open_file!(
		target_information_file_path,
		"Target information file not found.",
		"Failed to open target information file."
	);
	let target_information: BTreeMap<String, Vec<String>> = try_result!(
		serde_yaml::from_reader(target_information_file),
		"Failed to parse target information file."
	);
	let names = Rc::new(try_option!(
		target_information.get("names"),
		"Names not provided in target information file."
	).clone());
	let numbers = Rc::new(try_option!(
		target_information.get("numbers"),
		"Numbers not provided in target information file."
	).clone());
	let symbols = Rc::new(commonly_used::symbols());
	let common_texts = Rc::new(commonly_used::texts());
	let common_numbers = Rc::new(commonly_used::numbers());

	let get_combination = |c| -> Result<Box<dyn Combinations>, String> {
		match c {
			Some('n') => Ok(Box::new(NameCombinations::new(names.clone()))),
			Some('0') => Ok(Box::new(SequenceCombinations::new(numbers.clone()))),
			Some('$') => Ok(Box::new(ArrayCombinations::new(symbols.clone()))),
			Some('x') => Ok(Box::new(SequenceCombinations::new(common_texts.clone()))),
			Some('y') => Ok(Box::new(SequenceCombinations::new(common_numbers.clone()))),
			_ => Err("Failed to parse target information file.".to_owned())
		}
	};

	if let Some(patterns) = target_information.get("patterns") {
		let mut pattern_combinations: Vec<Box<dyn Combinations>> = Vec::new();

		for pattern in patterns {
			let pattern = pattern.trim();
			let combinations: Box<dyn Combinations>;
			if pattern.len() > 2 {
				let mut combine_combinations: CombineCombinations = CombineCombinations::new(
					get_combination(pattern.chars().nth(pattern.len() - 3))?,
					get_combination(pattern.chars().nth(pattern.len() - 1))?
				);
				if pattern.len() > 4 {
					let mut i = pattern.len() - 5;
					loop {
						combine_combinations = CombineCombinations::new(
							get_combination(pattern.chars().nth(i))?,
							Box::new(combine_combinations)
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
				combinations = get_combination(pattern.chars().nth(0))?;
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

	let password_list_file_reader = BufReader::new(try_open_file!(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	));

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
						passwords.push(current_password.clone());
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
