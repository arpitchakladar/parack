mod hash;
mod algorithms;

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
	targeted_guess(md5, target_information_file, "adfdasf");
}
