use std::fs::File;
use std::io;
use std::collections::HashMap;

pub trait Resolve<T> {
	fn resolve(self, error_message: &'static str) -> Result<T, &'static str>;
}

impl<T, U> Resolve<T> for Result<T, U> {
	fn resolve(self, error_message: &'static str) -> Result<T, &'static str> {
		match self {
			Ok(x) => Ok(x),
			Err(_) => Err(error_message)
		}
	}
}

impl<T> Resolve<T> for Option<T> {
	fn resolve(self, error_message: &'static str) -> Result<T, &'static str> {
		match self {
			Some(x) => Ok(x),
			None => Err(error_message)
		}
	}
}

pub fn open_file(path: &str, file_not_found_message: &'static str, failed_to_open_file_message: &'static str) -> Result<File, &'static str> {
	match File::open(path) {
		Ok(file) => Ok(file),
		Err(error) => match error.kind() {
			io::ErrorKind::NotFound => {
				Err(file_not_found_message)
			},
			_ => {
				Err(failed_to_open_file_message)
			}
		}
	}
}

pub fn parse_args<'a>(args: &'a Vec<String>, fields: Vec<(&'a str, &'static str)>) -> Result<HashMap<&'a str, &'a str>, &'static str> {
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
			return Err(error_message);
		}
	}

	Ok(res)
}

pub fn vec_from_slices<T: Clone, const N: usize>(slices: [&[T]; N]) -> Vec<T> {
	let mut vec = Vec::with_capacity(slices.iter().fold(0, |acc, x| acc + x.len()));
	for slice in slices {
		vec.extend_from_slice(slice);
	}
	vec
}
