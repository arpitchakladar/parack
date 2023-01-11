use std::option::Option;
use std::rc::Rc;

use crate::combinations::Combinations;

pub struct ArrayCombinations {
	array: Rc<Vec<Vec<u8>>>,
	index: usize
}

impl ArrayCombinations {
	pub fn new(array: Rc<Vec<Vec<u8>>>) -> Self {
		Self {
			array,
			index: 0
		}
	}
}

impl Iterator for ArrayCombinations {
	type Item = Vec<u8>;

	fn next(&mut self) -> Option<Self::Item> {
		self.index += 1;
		self.array.get(self.index - 1).cloned()
	}
}

impl Combinations for ArrayCombinations {
	fn reset(&mut self) {
		self.index = 0;
	}

	fn possibilities(&self) -> usize {
		self.array.len()
	}
}
