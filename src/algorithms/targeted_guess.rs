use std::option::Option;
use std::fs;
use std::collections::BTreeMap;

use serde_yaml;

trait Combination: Iterator<Item = String> {
	fn reset(&mut self);
}

struct NameCombinations<'a> {
	name: &'a str,
	count: usize,
}

impl<'a> NameCombinations<'a> {
	fn new(name: &'a str) -> Self {
		Self {
			name,
			count: 0
		}
	}
}

impl Iterator for NameCombinations<'_> {
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

impl Combination for NameCombinations<'_> {
	fn reset(&mut self) {
		self.count = 0;
	}
}

struct NumberCombinations<'a> {
	number: &'a str,
	start: usize,
	end: usize
}

impl<'a> NumberCombinations<'a> {
	fn new(number: &'a str) -> Self {
		Self {
			number,
			start: 0,
			end: number.len()
		}
	}
}

impl Iterator for NumberCombinations<'_> {
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

impl Combination for NumberCombinations<'_> {
	fn reset(&mut self) {
		self.start = 0;
		self.end = self.number.len();
	}
}

struct CombineCombinations<T1: Combination, T2: Combination> {
	permutations1: T1,
	permutations2: T2,
	current_permutation: String
}

impl<T1: Combination, T2: Combination> CombineCombinations<T1, T2> {
	fn new(mut permutations1: T1, permutations2: T2) -> Self {
		Self {
			current_permutation: permutations1.next().unwrap(),
			permutations1,
			permutations2
		}
	}

	fn add<T3: Combination>(self, permutation: T3) -> CombineCombinations<Self, T3> {
		CombineCombinations::new(self, permutation)
	}
}

impl<T1: Combination, T2: Combination> Iterator for CombineCombinations<T1, T2> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self.permutations2.next() {
			Some(ref permutation) => Some(self.current_permutation.clone() + permutation),
			None => {
				match self.permutations1.next() {
					Some(next_permutation) => {
						self.permutations2.reset();
						self.current_permutation = next_permutation;
						self.next()
					},
					None => None
				}
			}
		}
	}
}

impl<T1: Combination, T2: Combination> Combination for CombineCombinations<T1, T2> {
	fn reset(&mut self) {
		self.permutations1.reset();
		self.permutations2.reset();
		self.current_permutation = self.permutations1.next().unwrap();
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
