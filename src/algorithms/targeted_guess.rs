use std::fs::File;
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

fn generate_password_patterns(target_information_file_path: &str) -> Vec<Box<dyn Combinations>> {
	let target_information_file = File::open(target_information_file_path).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_reader(target_information_file).unwrap();
	let names = Rc::new(target_information.get("names").unwrap().clone());
	let numbers = Rc::new(target_information.get("numbers").unwrap().clone());
	let symbols = Rc::new(commonly_used::symbols());
	let common_texts = Rc::new(commonly_used::texts());
	let common_numbers = Rc::new(commonly_used::numbers());

	let mut patterns: Vec<Box<dyn Combinations>> = Vec::new();

	for pattern in target_information.get("patterns").unwrap() {
		let pattern = pattern.trim();
		let combinations: Box<dyn Combinations>;
		if pattern.len() > 2 {
			let get_combination = |i: usize| -> Rc<RefCell<dyn Combinations>> {
				match pattern.chars().nth(i) {
					Some('n') => rc_ref_cell!(NameCombinations::new(names.clone())),
					Some('0') => rc_ref_cell!(SequenceCombinations::new(numbers.clone())),
					Some('$') => rc_ref_cell!(ArrayCombinations::new(symbols.clone())),
					Some('x') => rc_ref_cell!(SequenceCombinations::new(common_texts.clone())),
					Some('y') => rc_ref_cell!(SequenceCombinations::new(common_numbers.clone())),
					_ => panic!("Invalid character.")
				}
			};
			let mut combine_combinations: CombineCombinations = CombineCombinations::new(
				get_combination(pattern.len() - 3),
				get_combination(pattern.len() - 1)
			);
			if pattern.len() > 4 {
				let mut i = pattern.len() - 5;
				loop {
					combine_combinations = CombineCombinations::new(
						get_combination(i),
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
				_ => panic!("Invalid character.")
			}
		}
		patterns.push(combinations);
	}

	patterns
}

pub fn targeted_guess(hash: HashFunction, target_information_file_path: &str, password_list_file: &str) {
	let mut patterns = generate_password_patterns(target_information_file_path);

	let password_list_file = File::open(password_list_file).unwrap();
	let password_list_file_reader = BufReader::new(password_list_file);

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
				for p in combinations {
					let current_password = {
						if let Some(salt) = salt {
							p.clone() + salt
						} else {
							p.clone()
						}
					};

					if password.eq_ignore_ascii_case(&hash(&current_password)) {
						println!("Found password {}", p);
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
}
