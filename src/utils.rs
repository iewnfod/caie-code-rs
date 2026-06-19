use colored::Colorize;

pub fn debug_print<T: ToString>(debug: bool, message: T) {
	if debug {
		println!("{}", message.to_string().purple());
	}
}
