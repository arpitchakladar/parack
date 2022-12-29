#[macro_export]
macro_rules! rc_ref_cell {
	($x:expr) => {
		Rc::new(RefCell::new($x))
	}
}

pub use rc_ref_cell;
