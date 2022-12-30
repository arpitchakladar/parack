use std::option::Option;
use std::fs;
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

pub fn targeted_guess(hash: HashFunction, target_information_file: &str, password_list_file: &str) {
	let file = fs::read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	drop(file);
	let patterns = target_information.get("patterns").unwrap();
	let names = Rc::new(target_information.get("names").unwrap().clone());
	let numbers = Rc::new(target_information.get("numbers").unwrap().clone());
	let symbols = Rc::new(commonly_used::symbols());
	let common_texts = Rc::new(commonly_used::texts());
	let common_numbers = Rc::new(commonly_used::numbers());

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
			for pattern in patterns {
				let pattern = pattern.trim();
				let combinations: Box<dyn Combinations>;
				if pattern.len() > 1 {
					let mut combine_combinations: Vec<Rc<RefCell<dyn Combinations>>> = Vec::new();
					let mut i = 0usize;
					while i < pattern.len() {
						match pattern.chars().nth(i) {
							Some('n') => combine_combinations.push(
								rc_ref_cell!(NameCombinations::new(names.clone()))
							),
							Some('0') => combine_combinations.push(
								rc_ref_cell!(SequenceCombinations::new(numbers.clone()))
							),
							Some('$') => combine_combinations.push(
								rc_ref_cell!(ArrayCombinations::new(symbols.clone()))
							),
							Some('x') => combine_combinations.push(
								rc_ref_cell!(SequenceCombinations::new(common_texts.clone()))
							),
							Some('y') => combine_combinations.push(
								rc_ref_cell!(SequenceCombinations::new(common_numbers.clone()))
							),
							_ => panic!("Invalid character.")
						}
						i += 2;
					}
					combinations = Box::new(CombineCombinations::combine(combine_combinations));
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

				let mut done = false;

				for p in combinations {
					let current_password = {
						if let Some(salt) = salt {
							p.clone() + salt
						} else {
							p.clone()
						}
					};

					if password.eq_ignore_ascii_case(&hash(&current_password)) {
						println!("Found password \x1b[1;32m{}\x1b[m", p);
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
