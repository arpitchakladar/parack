use std::option::Option;
use std::fs::read_to_string;
use std::collections::BTreeMap;

use serde_yaml;

pub fn targeted_guess<F>(hash: F, target_information_file: &str, password: &str) -> Option<String>
	where
		F:  Fn(&str) -> String
{
	let file = read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	if let Some(names) = target_information.get("names") {
		println!("{:?}", names);
	}
	None
}
