use std::option::Option;
use super::Combinations;

pub(super) struct NumberCombinations<'a> {
	number: &'a str,
	start: usize,
	end: usize
}

impl<'a> NumberCombinations<'a> {
	pub(super) fn new(number: &'a str) -> Self {
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

impl Combinations for NumberCombinations<'_> {
	fn reset(&mut self) {
		self.start = 0;
		self.end = self.number.len();
	}
}
