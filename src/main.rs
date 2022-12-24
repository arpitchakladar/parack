mod hash;
mod algorithms;

use hash::{md5};
use algorithms::{wordlist_search};

fn main() {
	let cracked_password = wordlist_search(md5, "~/.local/share/wordlists/rockyou.txt", "3f13d53e0eddce1f54f9b9d394b6bff0");
	println!("{:?}", cracked_password);
}
