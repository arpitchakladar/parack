use std::option::Option;
use super::Combinations;

pub(super) struct NameCombinations<'a> {
	name: &'a str,
	count: usize,
}

impl<'a> NameCombinations<'a> {
	pub(super) fn new(name: &'a str) -> Self {
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

impl Combinations for NameCombinations<'_> {
	fn reset(&mut self) {
		self.count = 0;
	}
}
