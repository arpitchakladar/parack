use std::option::Option;
use std::rc::Rc;

use crate::combinations::Combinations;

use crate::utils::append_vectors;

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

#[inline]
fn get_uppercase(bytes: &[u8]) -> Vec<u8> {
	let mut res = Vec::with_capacity(bytes.len());
	for byte in bytes {
		let result_byte = match *byte as char {
			'a'..='z' => byte - 32, // 'a' - 32 = 'A'
			_ => *byte
		};
		res.push(result_byte);
	}
	res
}

impl Iterator for NameCombinations {
	type Item = Vec<u8>;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		let name = &self.names[self.index];
		match self.count {
			1 => Some(name.to_owned()),
			2 => Some(append_vectors!(get_uppercase(&name[0..1]), [&name[1..]])),
			3 => Some(get_uppercase(&name)),
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
