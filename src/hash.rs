use digest::Digest;
use md2::Md2;
use md4::Md4;
use md5::Md5;
use sha1::Sha1;
use sha2::{
	Sha256,
	Sha512
};

pub type HashFunction = fn(&str, &str) -> String;

pub fn md2(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Md2::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}

pub fn md4(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Md4::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}

pub fn md5(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Md5::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}

pub fn sha1(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Sha1::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}

pub fn sha256(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Sha256::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}

pub fn sha512(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Sha512::new()
			.chain_update(password.as_bytes())
			.chain_update(hash.as_bytes())
			.finalize()
	)
}
