use std::option::Option;
use super::Combinations;

pub struct ArrayCombinations<'a> {
	array: &'a [String],
	index: usize
}

impl<'a> ArrayCombinations<'a> {
	pub fn new(array: &'a [String]) -> Self {
		Self {
			array,
			index: 0
		}
	}
}

impl Iterator for ArrayCombinations<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.index += 1;
		self.array.get(self.index - 1).cloned()
	}
}

impl Combinations for ArrayCombinations<'_> {
	fn reset(&mut self) {
		self.index = 0;
	}

	fn possibilities(&self) -> usize {
		self.array.len()
	}
}
