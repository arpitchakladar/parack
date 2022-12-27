use crate::combinations::{
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

pub fn targeted_guess<F>(hash: F, target_information_file: &str, password: &str) -> Option<String>
	where
		F:  Fn(&str) -> String
{
	let file = fs::read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	drop(file);
	if let Some(names) = target_information.get("names") {
		if let Some(numbers) = target_information.get("numbers") {
			let password_combinations = CombineCombinations::new(
				NameCombinations::new(&names),
				SequenceCombinations::new(&numbers)
			).add(NameCombinations::new(&names));
			for password in password_combinations {
				println!("{}", password);
			}
		}
	}
	None
}
