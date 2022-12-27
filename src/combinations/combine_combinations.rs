use std::option::Option;
use super::Combinations;

#[macro_export]
macro_rules! combine_combinations {
	($combinations:expr) => {
		$combinations
	};
	($combinations1:expr, $($combinations:expr),*) => {
		{
			CombineCombinations::new($combinations1, combine_combinations!($($combinations),*))
		}
	};
}

pub use combine_combinations;

pub struct CombineCombinations<T1: Combinations, T2: Combinations> {
	combinations1: T1,
	combinations2: T2,
	current_combination: String
}

impl<T1: Combinations, T2: Combinations> CombineCombinations<T1, T2> {
	pub fn new(mut combinations1: T1, combinations2: T2) -> Self {
		Self {
			current_combination: combinations1.next().unwrap(),
			combinations1,
			combinations2
		}
	}
}

impl<T1: Combinations, T2: Combinations> Iterator for CombineCombinations<T1, T2> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self.combinations2.next() {
			Some(ref combination) => Some(self.current_combination.clone() + combination),
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

impl<T1: Combinations, T2: Combinations> Combinations for CombineCombinations<T1, T2> {
	fn reset(&mut self) {
		self.combinations1.reset();
		self.combinations2.reset();
		self.current_combination = self.combinations1.next().unwrap();
	}

	fn possibilities(&self) -> usize {
		self.combinations1.possibilities() * self.combinations2.possibilities()
	}
}
