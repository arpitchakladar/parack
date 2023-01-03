use std::collections::HashMap;

#[macro_export]
macro_rules! try_result {
	($x:expr, $error_message:tt) => {
		match $x {
			Ok(x) => Ok(x),
			Err(_) => Err($error_message.to_owned())
		}?
	}
}

#[macro_export]
macro_rules! try_option {
	($x:expr, $error_message:tt) => {
		match $x {
			Some(x) => Ok(x),
			None => Err($error_message.to_owned())
		}?
	}
}

#[macro_export]
macro_rules! try_open_file {
	($x:tt, $file_not_found_message:tt, $failed_to_open_file_message:tt) => {
		match File::open($x) {
			Ok(file) => Ok(file),
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => {
					Err($file_not_found_message.to_owned())
				},
				_ => {
					Err($failed_to_open_file_message.to_owned())
				}
			}
		}?
	}
}

pub use try_result;
pub use try_option;
pub use try_open_file;

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
