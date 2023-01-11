use digest::Digest;
use md2::Md2;
use md4::Md4;

use openssl::hash::{Hasher, MessageDigest};

pub type HashFunction = fn(&[u8], &[u8]) -> Vec<u8>;

pub fn md2(password: &[u8], salt: &[u8]) -> Vec<u8> {
	Md2::new()
		.chain_update(password)
		.chain_update(salt)
		.finalize()
		.to_vec()
}

pub fn md4(password: &[u8], salt: &[u8]) -> Vec<u8> {
	Md4::new()
		.chain_update(password)
		.chain_update(salt)
		.finalize()
		.to_vec()
}

pub fn md5(password: &[u8], salt: &[u8]) -> Vec<u8> {
	let mut hasher = Hasher::new(MessageDigest::md5()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().to_vec()
}

pub fn sha1(password: &[u8], salt: &[u8]) -> Vec<u8> {
	let mut hasher = Hasher::new(MessageDigest::sha1()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().to_vec()
}

pub fn sha256(password: &[u8], salt: &[u8]) -> Vec<u8> {
	let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().to_vec()
}

pub fn sha512(password: &[u8], salt: &[u8]) -> Vec<u8> {
	let mut hasher = Hasher::new(MessageDigest::sha512()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().to_vec()
}
