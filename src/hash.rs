use digest::Digest;
use md2::Md2;
use md4::Md4;

use openssl::hash::{
	Hasher,
	MessageDigest
};

pub type CheckHash = fn(&[u8], &[u8], &[u8]) -> bool;

pub fn md2(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	Md2::new()
		.chain_update(password)
		.chain_update(salt)
		.finalize()
		.as_slice()
		.eq(compare)
}

pub fn md4(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	Md4::new()
		.chain_update(password)
		.chain_update(salt)
		.finalize()
		.as_slice()
		.eq(compare)
}

pub fn md5(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	let mut hasher = Hasher::new(MessageDigest::md5()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().eq(compare)
}

pub fn sha1(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	let mut hasher = Hasher::new(MessageDigest::sha1()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().eq(compare)
}

pub fn sha256(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().eq(compare)
}

pub fn sha512(password: &[u8], salt: &[u8], compare: &[u8]) -> bool {
	let mut hasher = Hasher::new(MessageDigest::sha512()).unwrap();
	hasher.update(password).unwrap();
	hasher.update(salt).unwrap();
	hasher.finish().unwrap().eq(compare)
}
