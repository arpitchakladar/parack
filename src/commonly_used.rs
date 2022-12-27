pub fn symbols(priority: u8) -> Vec<String> {
	match priority {
		0 => vec![
			"@".to_string(),
			"$".to_string(),
			"!".to_string(),
			"#".to_string(),
			"%".to_string(),
			"^".to_string(),
			"*".to_string(),
			"-".to_string(),
			"+".to_string(),
			"_".to_string()
		],
		1 => vec![
			"@".to_string(),
			"$".to_string(),
			"!".to_string(),
			"#".to_string()
		],
		2 => vec![
			"%".to_string(),
			"^".to_string(),
			"*".to_string(),
			"-".to_string(),
			"+".to_string(),
			"_".to_string()
		],
		// TODO: Add more symbols,
		_ => Vec::new()
	}
}
