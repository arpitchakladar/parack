use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

use crate::combinations::{
	combine_combinations,
	Combinations,
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

pub fn targeted_guess<F>(hash: F, target_information_file: &str, password: &str) -> Option<String>
	where
		F:  Fn(&str) -> String
{
	let file = fs::read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	drop(file);
	let names = target_information.get("names").unwrap();
	let numbers = target_information.get("numbers").unwrap();
	let password_combinations = combine_combinations![
		NameCombinations::new(&names),
		SequenceCombinations::new(&numbers),
		NameCombinations::new(&names)
	];
	println!("Total number of possible passwords combinations is {}", password_combinations.possibilities());
	None
}
