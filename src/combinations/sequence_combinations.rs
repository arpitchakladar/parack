use std::option::Option;
use super::Combinations;

pub struct SequenceCombinations<'a> {
	sequences: &'a [String],
	index: usize,
	start: usize,
	end: usize
}

impl<'a> SequenceCombinations<'a> {
	pub fn new(sequences: &'a [String]) -> Self {
		Self {
			sequences,
			index: 0,
			start: 0,
			end: sequences[0].len()
		}
	}
}

impl Iterator for SequenceCombinations<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let sequence = &self.sequences[self.index];
		if self.start >= self.end {
			if self.start >= sequence.len() {
				if self.index < (self.sequences.len() - 1) {
					self.index += 1;
					self.start = 0;
					self.end = self.sequences[self.index].len();
					self.next()
				} else {
					None
				}
			} else {
				self.start += 1;
				self.end = sequence.len();
				self.next()
			}
		} else {
			self.end -= 1;
			Some(sequence[self.start..(self.end + 1)].to_string())
		}
	}
}

impl Combinations for SequenceCombinations<'_> {
	fn reset(&mut self) {
		self.start = 0;
		self.index = 0;
		self.end = self.sequences[0].len();
	}
}
