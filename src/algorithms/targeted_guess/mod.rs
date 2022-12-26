mod combinations;
mod combine_combinations;
mod name_combinations;
mod number_combinations;

use combinations::Combinations;
use combine_combinations::CombineCombinations;
use name_combinations::NameCombinations;
use number_combinations::NumberCombinations;

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
				NameCombinations::new(&names[0]),
				NumberCombinations::new(&numbers[0])
			).add(NameCombinations::new(&names[1]));
			for passwords in password_combinations {
				println!("{}", passwords);
			}
		}
	}
	None
}
