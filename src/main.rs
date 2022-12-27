pub mod hash;
pub mod commonly_used;
pub mod algorithms;
pub mod combinations;

use std::{
	env, path::Path
};
use hash::{md5};
use algorithms::{targeted_guess};

fn main() {
	let current_dir = env::current_dir()
		.unwrap()
		.display()
		.to_string();
	let target_information_file_path = Path::new(&current_dir)
		.join("tests/targets/t1.yml");
	let target_information_file = target_information_file_path
		.to_str()
		.unwrap();
	if let Some(password) = targeted_guess(md5, target_information_file, "b608d4bd68ff987187a065ac45833550") {
		println!("{}", password);
	}
}
