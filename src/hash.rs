use md5::{
	compute as compute_md5
};
use sha2::{
	Sha256,
	Sha512,
	Digest
};

pub fn md5(password: &str) -> String {
	format!("{:x}", compute_md5(password.as_bytes()))
}

pub fn sha256(password: &str) -> String {
	format!(
		"{:x}",
		Sha256::new()
			.chain_update(password.as_bytes())
			.finalize()
	)
}

pub fn sha512(password: &str) -> String {
	format!(
		"{:x}",
		Sha512::new()
			.chain_update(password.as_bytes())
			.finalize()
	)
}
