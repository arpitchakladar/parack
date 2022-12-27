pub trait Combinations: Iterator<Item = String> {
	fn reset(&mut self);
	fn possibilities(&self) -> usize;
}

mod combine_combinations;
mod name_combinations;
mod sequence_combinations;

pub use combine_combinations::{
	combine_combinations,
	CombineCombinations
};
pub use name_combinations::NameCombinations;
pub use sequence_combinations::SequenceCombinations;
