use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

use crate::commonly_used;
use crate::hash::HashFunction;
use crate::combinations::{
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

	macro_rules! generate_combinations {
		(numbers) => {
			{
				SequenceCombinations::new(&numbers)
			}
		};
		(symbols) => {
			{
				ArrayCombinations::new(&symbols)
			}
		};
		(names) => {
			{
				NameCombinations::new(&names)
			}
		};
		(numbers-$($x:tt)-*) => {
			{
				CombineCombinations::new(SequenceCombinations::new(&numbers), generate_combinations!($($x)-*))
			}
		};
		(symbols-$($x:tt)-*) => {
			{
				CombineCombinations::new(ArrayCombinations::new(&symbols), generate_combinations!($($x)-*))
			}
		};
		(names-$($x:tt)-*) => {
			{
				CombineCombinations::new(NameCombinations::new(&names), generate_combinations!($($x)-*))
			}
		};
	}

	macro_rules! check_combinations {
		($($($x:tt)-*),*) => {
			$(
				let combinations = generate_combinations!($($x)-*);
				for p in combinations {
					if hash(&p).eq_ignore_ascii_case(password) {
						return Some(p.to_string());
					}
				}
			)*
		}
	}

	check_combinations![
		names,
		numbers,
		names-numbers,
		numbers-names,
		names-symbols,
		names-symbols-numbers,
		names-symbols-names
	];

	None
}
