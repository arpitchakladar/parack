pub(super) trait Combinations: Iterator<Item = String> {
	fn reset(&mut self);
}
