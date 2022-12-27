use std::option::Option;
use super::Combinations;

pub struct NameCombinations<'a> {
	names: &'a [String],
	count: usize,
	index: usize
}

impl<'a> NameCombinations<'a> {
	pub fn new(names: &'a [String]) -> Self {
		Self {
			names,
			count: 0,
			index: 0
		}
	}
}

impl Iterator for NameCombinations<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		let name = &self.names[self.index];
		match self.count {
			1 => Some(name.to_string()),
			2 => Some(name[0..1].to_uppercase() + &name[1usize..]),
			3 => Some(name.to_uppercase()),
			_ => {
				if self.index < (self.names.len() - 1) {
					self.index += 1;
					self.count = 0;
					self.next()
				} else {
					None
				}
			}
		}
	}
}

impl Combinations for NameCombinations<'_> {
	fn reset(&mut self) {
		self.count = 0;
		self.index = 0;
	}
}
