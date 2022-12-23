mod hash;

use hash::{md5};

fn main() {
	println!("{}", md5("Hello, world!"));
}
