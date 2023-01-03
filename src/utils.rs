use std::fs::File;
use std::io;
use std::collections::HashMap;

pub trait Resolve<T> {
	fn resolve(self, error_message: &str) -> Result<T, String>;
}

impl<T1, T2> Resolve<T1> for Result<T1, T2> {
	fn resolve(self, error_message: &str) -> Result<T1, String> {
		match self {
			Ok(x) => Ok(x),
			Err(_) => Err(error_message.to_owned())
		}
	}
}

impl<T> Resolve<T> for Option<T> {
	fn resolve(self, error_message: &str) -> Result<T, String> {
		match self {
			Some(x) => Ok(x),
			None => Err(error_message.to_owned())
		}
	}
}

pub fn open_file(path: &str, file_not_found_message: &str, failed_to_open_file_message: &str) -> Result<File, String> {
	match File::open(path) {
		Ok(file) => Ok(file),
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				Err(file_not_found_message.to_owned())
			},
			_ => {
				Err(failed_to_open_file_message.to_owned())
			}
		}
	}
}

pub fn parse_args<'a>(args: &'a Vec<String>, fields: Vec<(&'a str, &'a str)>) -> Result<HashMap<&'a str, &'a str>, String> {
	let mut res: HashMap<&str, &str> = HashMap::with_capacity(fields.len());
	for i in 0..args.len() {
		for (command_line_arg, _) in &fields {
			if args[i].trim().eq_ignore_ascii_case(command_line_arg) {
				res.insert(command_line_arg, &args[i + 1]);
			}
		}
	}

	for (command_line_arg, error_message) in fields {
		if !res.contains_key(command_line_arg) {
			return Err(error_message.to_owned());
		}
	}

	Ok(res)
}
