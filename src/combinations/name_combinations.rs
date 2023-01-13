use std::option::Option;
use std::rc::Rc;
use std::ops::Range;

use crate::combinations::Combinations;

#[derive(Debug)]
pub struct NameCombinations {
	names: Rc<Vec<Vec<u8>>>,
	count: usize,
	index: usize
}

impl NameCombinations {
	pub fn new(names: Rc<Vec<Vec<u8>>>) -> Self {
		Self {
			names,
			count: 0,
			index: 0
		}
	}
}

fn get_uppercase(name: &[u8], range: Range<usize>) -> Vec<u8> {
	let mut res = Vec::with_capacity(name.len());
	for byte in &name[0..range.start] {
		res.push(*byte);
	}
	for byte in &name[range.clone()] {
		res.push(match *byte as char {
			'a'..='z' => *byte - 32, // 'a' - 32 = 'A'
			_ => *byte
		});
	}
	for byte in &name[range.end..name.len()] {
		res.push(*byte);
	}
	res
}

impl Iterator for NameCombinations {
	type Item = Vec<u8>;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		let name = &self.names[self.index];
		match self.count {
			1 => Some(name.clone()),
			2 => Some(get_uppercase(&name, 0..1)),
			3 => Some(get_uppercase(&name, 0..name.len())),
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

impl Combinations for NameCombinations {
	fn reset(&mut self) {
		self.count = 0;
		self.index = 0;
	}

	fn possibilities(&self) -> usize {
		self.names.len() * 3
	}
}
