use std::option::Option;

use super::Combinations;

pub struct CombineCombinations {
	combinations1: Box<dyn Combinations>,
	combinations2: Box<dyn Combinations>,
	current_combination: Vec<u8>
}

impl CombineCombinations {
	pub fn new(mut combinations1: Box<dyn Combinations>, combinations2: Box<dyn Combinations>) -> Self {
		Self {
			current_combination: combinations1.next().unwrap(),
			combinations1,
			combinations2
		}
	}
}

impl Iterator for CombineCombinations {
	type Item = Vec<u8>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.combinations2.next() {
			Some(ref combination) => {
				let mut current_combination = self.current_combination.to_owned();
				current_combination.extend_from_slice(combination);
				Some(current_combination)
			},
			None => {
				match self.combinations1.next() {
					Some(next_combination) => {
						self.combinations2.reset();
						self.current_combination = next_combination;
						self.next()
					},
					None => None
				}
			}
		}
	}
}

impl Combinations for CombineCombinations {
	fn reset(&mut self) {
		self.combinations1.reset();
		self.combinations2.reset();
		self.current_combination = self.combinations1.next().unwrap();
	}

	fn possibilities(&self) -> usize {
		self.combinations1.possibilities() * self.combinations2.possibilities()
	}
}
