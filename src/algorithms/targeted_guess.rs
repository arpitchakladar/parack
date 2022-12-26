use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

struct NamePermutations<'a> {
	name: &'a str,
	count: usize,
}

impl<'a> NamePermutations<'a> {
	fn new(name: &'a str) -> Self {
		Self {
			name,
			count: 0
		}
	}
}

impl Iterator for NamePermutations<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		match self.count {
			1 => Some(self.name.to_string()),
			2 => Some(self.name[0..1].to_uppercase() + &self.name[1usize..]),
			3 => Some(self.name.to_uppercase()),
			4 => Some(self.name[0..1].to_string() + &self.name[1usize..].to_uppercase()),
			_ => None
		}
	}
}

struct NumberPermutations<'a> {
	number: &'a str,
	start: usize,
	end: usize
}

impl<'a> NumberPermutations<'a> {
	fn new(number: &'a str) -> Self {
		Self {
			number,
			start: 0,
			end: number.len()
		}
	}
}

impl Iterator for NumberPermutations<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.start >= self.end {
			if self.start >= self.number.len() {
				None
			} else {
				self.start += 1;
				self.end = self.number.len();
				self.next()
			}
		} else {
			self.end -= 1;
			Some(self.number[self.start..(self.end + 1)].to_string())
		}
	}
}

pub fn targeted_guess<F>(hash: F, target_information_file: &str, password: &str) -> Option<String>
	where
		F:  Fn(&str) -> String
{
	let file = fs::read_to_string(target_information_file).unwrap();
	let target_information: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&file).unwrap();
	drop(file);
	if let Some(names) = target_information.get("names") {
		if let Some(numbers) = target_information.get("numbers") {
			for name in NamePermutations::new(&names[0]) {
				for number in NumberPermutations::new(&numbers[0]) {
					println!("{}{}", name, number);
				}
			}
		}
	}
	None
}
