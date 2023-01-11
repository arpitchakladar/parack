use digest::Digest;
use md2::Md2;
use md4::Md4;

use openssl::hash::{Hasher, hash, MessageDigest};

use crate::utils::hex_from_bytes;

pub type HashFunction = fn(&str, &str) -> String;

pub fn md2(password: &str, salt: &str) -> String {
	format!(
		"{:x}",
		Md2::new()
			.chain_update(password.as_bytes())
			.chain_update(salt.as_bytes())
			.finalize()
	)
}

pub fn md4(password: &str, salt: &str) -> String {
	format!(
		"{:x}",
		Md4::new()
			.chain_update(password.as_bytes())
			.chain_update(salt.as_bytes())
			.finalize()
	)
}

pub fn md5(password: &str, salt: &str) -> String {
	let mut hasher = Hasher::new(MessageDigest::md5()).unwrap();
	hasher.update(password.as_bytes()).unwrap();
	hasher.update(salt.as_bytes()).unwrap();
	hex_from_bytes(&hasher.finish().unwrap().to_vec())
}

pub fn sha1(password: &str, salt: &str) -> String {
	let mut hasher = Hasher::new(MessageDigest::sha1()).unwrap();
	hasher.update(password.as_bytes()).unwrap();
	hasher.update(salt.as_bytes()).unwrap();
	hex_from_bytes(&hasher.finish().unwrap().to_vec())
}

pub fn sha256(password: &str, salt: &str) -> String {
	let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
	hasher.update(password.as_bytes()).unwrap();
	hasher.update(salt.as_bytes()).unwrap();
	hex_from_bytes(&hasher.finish().unwrap().to_vec())
}

pub fn sha512(password: &str, salt: &str) -> String {
	let mut hasher = Hasher::new(MessageDigest::sha512()).unwrap();
	hasher.update(password.as_bytes()).unwrap();
	hasher.update(salt.as_bytes()).unwrap();
	hex_from_bytes(&hasher.finish().unwrap().to_vec())
}
