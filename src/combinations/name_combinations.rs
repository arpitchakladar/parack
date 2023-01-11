use std::option::Option;
use std::rc::Rc;

use super::Combinations;

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
		let b = match byte {
			97..=122 => byte - 32,
			_ => byte.clone()
		};
		res.push(b);
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
			2 => {
				let mut current_name = get_uppercase(&name[0..1]);
				current_name.extend_from_slice(&name[1usize..]);
				Some(current_name)
			},
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
