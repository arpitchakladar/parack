pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::{
	env, path::Path
};
use hash::{md5};
use algorithms::{targeted_guess};
use std::time::{Instant};

fn main() {
	let current = Instant::now();
	let current_dir = env::current_dir()
		.unwrap()
		.display()
		.to_string();

	targeted_guess(
		md5,
		&format!(
			"{}",
			Path::new(&current_dir)
				.join("tests/targets/t1.yml")
				.display()
		),
		&format!(
			"{}",
			Path::new(&current_dir)
				.join("tests/password_lists/p1.txt")
				.display()
		)
	);
		
	let duration = current.elapsed();
   println!("Time elapsed is: {:?}", duration);
}
