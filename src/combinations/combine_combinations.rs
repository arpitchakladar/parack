use std::option::Option;
use super::Combinations;

pub struct CombineCombinations<T1: Combinations, T2: Combinations> {
	permutations1: T1,
	permutations2: T2,
	current_permutation: String
}

impl<T1: Combinations, T2: Combinations> CombineCombinations<T1, T2> {
	pub fn new(mut permutations1: T1, permutations2: T2) -> Self {
		Self {
			current_permutation: permutations1.next().unwrap(),
			permutations1,
			permutations2
		}
	}

	pub fn add<T3: Combinations>(self, permutation: T3) -> CombineCombinations<Self, T3> {
		CombineCombinations::new(self, permutation)
	}
}

impl<T1: Combinations, T2: Combinations> Iterator for CombineCombinations<T1, T2> {
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

impl<T1: Combinations, T2: Combinations> Combinations for CombineCombinations<T1, T2> {
	fn reset(&mut self) {
		self.permutations1.reset();
		self.permutations2.reset();
		self.current_permutation = self.permutations1.next().unwrap();
	}
}
