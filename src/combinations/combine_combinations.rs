use std::option::Option;
use std::rc::Rc;
use std::cell::RefCell;

use super::Combinations;

pub struct CombineCombinations {
	combinations1: Rc<RefCell<dyn Combinations>>,
	combinations2: Rc<RefCell<dyn Combinations>>,
	current_combination: String
}

impl CombineCombinations {
	pub fn new(combinations1: Rc<RefCell<dyn Combinations>>, combinations2: Rc<RefCell<dyn Combinations>>) -> Self {
		let current_combination = combinations1.borrow_mut().next().unwrap();
		Self {
			combinations1,
			combinations2,
			current_combination
		}
	}
}

impl Iterator for CombineCombinations {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let mut combinations1 = self.combinations1.borrow_mut();
		let mut combinations2 = self.combinations2.borrow_mut();
		match combinations2.next() {
			Some(ref combination) => Some(self.current_combination.clone() + combination),
			None => {
				match combinations1.next() {
					Some(next_combination) => {
						combinations2.reset();
						self.current_combination = next_combination;
						drop(combinations1);
						drop(combinations2);
						self.next()
					},
					None => None
				}
			}
		}
	}
}

impl Combinations for CombineCombinations {
	fn reset(&mut self) {
		let mut combinations1 = self.combinations1.borrow_mut();
		let mut combinations2 = self.combinations2.borrow_mut();
		combinations1.reset();
		combinations2.reset();
		self.current_combination = combinations1.next().unwrap();
	}

	fn possibilities(&self) -> usize {
		self.combinations1.borrow().possibilities() * self.combinations2.borrow().possibilities()
	}
}
