use digest::{
	Digest
};
use md5::{
	Md5
};
use sha2::{
	Sha256,
	Sha512
};

pub type HashFunction = fn(&str, &str) -> String;

pub fn md5(password: &str, hash: &str) -> String {
	format!(
		"{:x}",
		Md5::new()
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
