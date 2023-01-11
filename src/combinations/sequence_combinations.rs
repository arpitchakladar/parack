use std::option::Option;
use std::rc::Rc;

use super::Combinations;

pub struct SequenceCombinations {
	sequences: Rc<Vec<Vec<u8>>>,
	index: usize,
	start: usize,
	end: usize
}

impl SequenceCombinations {
	pub fn new(sequences: Rc<Vec<Vec<u8>>>) -> Self {
		Self {
			end: sequences[0].len(),
			sequences,
			index: 0,
			start: 0
		}
	}
}

impl Iterator for SequenceCombinations {
	type Item = Vec<u8>;

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
			Some(sequence[self.start..(self.end + 1)].to_owned())
		}
	}
}

impl Combinations for SequenceCombinations {
	fn reset(&mut self) {
		self.start = 0;
		self.index = 0;
		self.end = self.sequences[0].len();
	}

	fn possibilities(&self) -> usize {
		self.sequences.iter().fold(0, |acc, x| acc + (x.len() * (x.len() + 1))/2)
	}
}
