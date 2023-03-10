use std::io::{
	BufRead,
	BufReader
};
use std::rc::Rc;
use std::collections::HashMap;

use serde_yaml;

use crate::commonly_used;
use crate::utils::{
	Resolve,
	open_file
};
use crate::hash::CheckHash;
use crate::combinations::{
	Combinations,
	ArrayCombinations,
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

fn generate_password_patterns(target_information_file_path: &str) -> Result<Vec<Box<dyn Combinations>>, &'static str> {
	let target_information: HashMap<String, Vec<String>> = serde_yaml::from_reader(open_file(
		target_information_file_path,
		"Target information file not found.",
		"Failed to open target information file."
	)?).resolve("Failed to parse target information file.")?;
	let names = Rc::new(
		target_information.get("names")
			.resolve("Names not provided in target information file.")?
			.clone()
			.iter()
			.cloned()
			.map(|x| x.into_bytes())
			.collect::<Vec<Vec<u8>>>()
	);
	let numbers = Rc::new(
		target_information.get("numbers")
			.resolve("Numbers not provided in target information file.")?
			.clone()
			.iter()
			.cloned()
			.map(|x| x.into_bytes())
			.collect::<Vec<Vec<u8>>>()
	);
	let symbols = Rc::new(commonly_used::symbols());
	let common_texts = Rc::new(commonly_used::texts());
	let common_numbers = Rc::new(commonly_used::numbers());

	let get_combination = |pattern: &str, i: usize| -> Result<Box<dyn Combinations>, &'static str> {
		match pattern.chars().nth(i) {
			Some('n') => Ok(Box::new(NameCombinations::new(names.clone()))),
			Some('0') => Ok(Box::new(SequenceCombinations::new(numbers.clone()))),
			Some('$') => Ok(Box::new(ArrayCombinations::new(symbols.clone()))),
			Some('x') => Ok(Box::new(SequenceCombinations::new(common_texts.clone()))),
			Some('y') => Ok(Box::new(SequenceCombinations::new(common_numbers.clone()))),
			_ => Err("Failed to parse target information file.")
		}
	};

	if let Some(patterns) = target_information.get("patterns") {
		let mut pattern_combinations: Vec<Box<dyn Combinations>> = Vec::new();

		for pattern in patterns {
			let pattern = pattern.trim();
			let combinations = {
				if pattern.len() > 2 {
					let mut combine_combinations = CombineCombinations::new(
						get_combination(pattern, pattern.len() - 3)?,
						get_combination(pattern, pattern.len() - 1)?
					);
					if pattern.len() > 4 {
						let mut i = pattern.len() - 5;
						loop {
							combine_combinations = CombineCombinations::new(
								get_combination(pattern, i)?,
								Box::new(combine_combinations)
							);
							if i < 2 {
								break;
							} else {
								i -= 2;
							}
						}
					}
					Box::new(combine_combinations)
				} else {
					get_combination(pattern, 0)?
				}
			};
			pattern_combinations.push(combinations);
		}

		Ok(pattern_combinations)
	} else {
		Err("Patterns not provided in target information file.")
	}
}

pub fn targeted_guess(hash: CheckHash, target_information_file_path: &str, password_list_file_path: &str) -> Result<HashMap<String, String>, &'static str> {
	let mut patterns = generate_password_patterns(target_information_file_path)?;

	let password_list_file_reader = BufReader::new(open_file(
		password_list_file_path,
		"Password list file not found.",
		"Failed to open password list file."
	)?);

	let mut passwords = HashMap::new();
	'outer: for password in password_list_file_reader.lines() {
		if let Ok(original_password) = password {
			let splitted_password: Vec<&str> = original_password.split(":").collect();
			let password = hex::decode(splitted_password[0].trim())
				.resolve("Hash in password list file is not valid hexadecimal.")?;
			let salt = {
				if splitted_password.len() > 1 {
					splitted_password[1].trim()
				} else {
					""
				}
			}.as_bytes();

			for combinations in &mut patterns {
				combinations.reset();
				for current_password in combinations {
					if hash(&current_password, salt, &password) {
						let result_password = unsafe {
							String::from_utf8_unchecked(current_password)
						};
						passwords.insert(original_password, result_password);
						continue 'outer;
					}
				}
			}
		}
	}

	Ok(passwords)
}
