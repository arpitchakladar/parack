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
	let names = target_information.get("names")?;
	let numbers = target_information.get("numbers")?;
	let symbols = commonly_used::symbols();
	let common_texts = commonly_used::texts();
	let common_numbers = commonly_used::numbers();

	let mut total_possibilities = 0usize;
	macro_rules! check_combinations {
		($($combinations:expr),*) => {
			$(
				let combinations = $combinations;
				total_possibilities += combinations.possibilities();
				for p in combinations {
					if hash(&p).eq_ignore_ascii_case(password) {
						return Some(p);
					}
				}
			)*
		};
	}

	check_combinations![
		NameCombinations::new(&names),
		SequenceCombinations::new(&common_texts),
		SequenceCombinations::new(&numbers),
		SequenceCombinations::new(&common_numbers),
		combine_combinations![
			NameCombinations::new(&names),
			SequenceCombinations::new(&numbers)
		],
		combine_combinations![
			NameCombinations::new(&names),
			SequenceCombinations::new(&common_numbers)
		],
		combine_combinations![
			SequenceCombinations::new(&common_texts),
			SequenceCombinations::new(&common_numbers)
		],
		combine_combinations![
			SequenceCombinations::new(&numbers),
			NameCombinations::new(&names)
		],
		combine_combinations![
			SequenceCombinations::new(&common_numbers),
			NameCombinations::new(&names)
		],
		combine_combinations![
			SequenceCombinations::new(&common_numbers),
			SequenceCombinations::new(&common_texts)
		],
		combine_combinations![
			NameCombinations::new(&names),
			ArrayCombinations::new(&symbols)
		],
		combine_combinations![
			SequenceCombinations::new(&common_texts),
			ArrayCombinations::new(&symbols)
		],
		combine_combinations![
			SequenceCombinations::new(&numbers),
			ArrayCombinations::new(&symbols)
		],
		combine_combinations![
			SequenceCombinations::new(&common_numbers),
			ArrayCombinations::new(&symbols)
		],
		combine_combinations![
			NameCombinations::new(&names),
			ArrayCombinations::new(&symbols),
			SequenceCombinations::new(&numbers)
		],
		combine_combinations![
			NameCombinations::new(&names),
			ArrayCombinations::new(&symbols),
			SequenceCombinations::new(&common_numbers)
		]
	];

	None
}
