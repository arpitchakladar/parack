pub fn symbols() -> Vec<Vec<u8>> {
	vec![
		b"@".to_vec(),
		b"!".to_vec(),
		b"$".to_vec(),
		b"_".to_vec(),
		b"_".to_vec(),
		b".".to_vec(),
		b"&".to_vec(),
		b"#".to_vec(),
		b"*".to_vec(),
		b"-".to_vec(),
		b"+".to_vec(),
		b"%".to_vec(),
		b"^".to_vec()
	]
}

pub fn texts() -> Vec<Vec<u8>> {
	vec![
		b"qwertyuiop".to_vec(),
		b"asdfghjkl".to_vec(),
		b"zxcvbnm".to_vec(),
		b"qazwsx".to_vec(),
		b"1qaz2wsx".to_vec(),
		b"abcdefghijklmnopqrstuvwxyz".to_vec()
	]
}

pub fn numbers() -> Vec<Vec<u8>> {
	vec![
		b"1234567890".to_vec(),
		b"0123456789".to_vec(),
		b"0987654321".to_vec(),
		b"9876543210".to_vec()
	]
}
