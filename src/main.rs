pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;

use std::{
	env, path::Path
};
use hash::{md5};
use algorithms::{targeted_guess};
use std::time::{Instant};

fn main() {
	let current_dir = env::current_dir()
		.unwrap()
		.display()
		.to_string();
	
	let current = Instant::now();
	let targeted_guess_result = targeted_guess(
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
				.join("tests/password_lists/pl2.txt")
				.display()
		)
	);

	match targeted_guess_result {
		Ok(passwords) => println!("Found passwords {:?}", passwords),
		Err(message) => println!("{}", message)
	}

	println!("Time elapsed is: {:?}", current.elapsed());
}
