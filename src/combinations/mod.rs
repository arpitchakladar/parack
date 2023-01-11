pub trait Combinations: Iterator<Item = Vec<u8>> {
	fn reset(&mut self);
	fn possibilities(&self) -> usize;
}

mod combine_combinations;
mod array_combinations;
mod name_combinations;
mod sequence_combinations;

pub use combine_combinations::{
	CombineCombinations
};
pub use array_combinations::ArrayCombinations;
pub use name_combinations::NameCombinations;
pub use sequence_combinations::SequenceCombinations;
