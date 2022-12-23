use md5;

fn main() {
	println!("{:x}", md5::compute(b"Hello, world!"));
}
