pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;
pub mod utils;

use std::{
	env, path::Path
};
use hash::{sha256};
use algorithms::{wordlist_search};
use std::time::{Instant};

fn main() {
	let current = Instant::now();
	let current_dir = env::current_dir()
		.unwrap()
		.display()
		.to_string();
	
	let wordlist_search_result = wordlist_search(
		sha256,
		&format!(
			"{}",
			Path::new(&current_dir)
				.join("tests/wordlists/wl1.txt")
				.display()
		),
		&format!(
			"{}",
			Path::new(&current_dir)
				.join("tests/password_lists/pl1.txt")
				.display()
		)
	);

	match wordlist_search_result {
		Ok(passwords) => {
			println!("Found passwords {:?}", passwords);
		},
		Err(message) => println!("{}", message)
	}

	println!("Time elapsed is: {:?}", current.elapsed());
}
