use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

use crate::commonly_used;
use crate::hash::HashFunction;
use crate::combinations::{
	combine_combinations,
	Combinations,
	ArrayCombinations,
	NameCombinations,
	SequenceCombinations,
	CombineCombinations
};

pub fn targeted_guess(hash: HashFunction, target_information_file: &str, password: &str) -> Option<String> {
	let file = fs::read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	drop(file);
	let names = target_information.get("names").unwrap();
	let numbers = target_information.get("numbers").unwrap();
	let symbols = commonly_used::symbols(1);
	let password_combinations = combine_combinations![
		NameCombinations::new(&names),
		ArrayCombinations::new(&symbols),
		SequenceCombinations::new(&numbers)
	];
	let possibilities = password_combinations.possibilities();
//	for password in password_combinations {
//		println!("{}", hash(&password));
//	}
	println!("Total number of possible passwords combinations is {}", possibilities);
	None
}
