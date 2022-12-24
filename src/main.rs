mod hash;
mod algorithms;

use std::{
	env, path::Path
};
use hash::{md5};
use algorithms::{wordlist_search};

fn main() {
	if let Some(home_path) = env::home_dir() {
		if let Some(ref wordlist_file) = Path::new(&format!("{}/.local/share/wordlists/rockyou.txt", home_path.display())).to_str() {
			let target_hash = "d8578edf8458ce06fbc5bb76a58c5ca4";
			if let Some(cracked_password) = wordlist_search(md5, wordlist_file, target_hash) {
				println!("{}", cracked_password);
			}
		}
	}
}
