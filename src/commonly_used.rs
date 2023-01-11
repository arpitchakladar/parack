pub fn symbols() -> Vec<Vec<u8>> {
	vec![
		"@".to_string().into_bytes(),
		"!".to_string().into_bytes(),
		"$".to_string().into_bytes(),
		"_".to_string().into_bytes(),
		"_".to_string().into_bytes(),
		".".to_string().into_bytes(),
		"&".to_string().into_bytes(),
		"#".to_string().into_bytes(),
		"*".to_string().into_bytes(),
		"-".to_string().into_bytes(),
		"+".to_string().into_bytes(),
		"%".to_string().into_bytes(),
		"^".to_string().into_bytes()
	]
}

pub fn texts() -> Vec<Vec<u8>> {
	vec![
		"qwertyuiop".to_string().into_bytes(),
		"asdfghjkl".to_string().into_bytes(),
		"zxcvbnm".to_string().into_bytes(),
		"qazwsx".to_string().into_bytes(),
		"1qaz2wsx".to_string().into_bytes(),
		"abcdefghijklmnopqrstuvwxyz".to_string().into_bytes()
	]
}

pub fn numbers() -> Vec<Vec<u8>> {
	vec![
		"1234567890".to_string().into_bytes(),
		"0123456789".to_string().into_bytes(),
		"0987654321".to_string().into_bytes(),
		"9876543210".to_string().into_bytes()
	]
}
