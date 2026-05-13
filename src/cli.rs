use crate::Interpreter;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ArgOption {
	short: &'static str,
	long: &'static str,
	description: Option<&'static str>,
}

const HELP_ARG_OPTION: ArgOption = ArgOption {
	short: "-h",
	long: "--help",
	description: Some("Show this help message"),
};

const VERSION_ARG_OPTION: ArgOption = ArgOption {
	short: "-v",
	long: "--version",
	description: Some("Show version information"),
};

const DEBUG_ARG_OPTION: ArgOption = ArgOption {
	short: "-d",
	long: "--debug",
	description: Some("Enable debug mode"),
};

const ARG_OPTIONS: &[ArgOption] = &[
	HELP_ARG_OPTION,
	VERSION_ARG_OPTION,
	DEBUG_ARG_OPTION,
];

fn version() {
	println!("CAIE Pseudocode Interpreter v{}", env!("CARGO_PKG_VERSION"));
}

fn head() {
	version();
	println!("Repository at https://github.com/iewnfod/caie-code-rs/");
	println!("Licensed under the MPL-2.0 License.");
}

fn help() {
	head();
	println!();
	println!("Usage: cpc [file_path] [options]");
	println!();
	println!("Options:");
	ARG_OPTIONS.iter().for_each(|opt| {
		if let Some(desc) = opt.description {
			println!("  {:<20} {}", format!("{}, {}", opt.short, opt.long), desc);
		} else {
			println!("  {:<20}", format!("{}, {}", opt.short, opt.long));
		}
	});
}

fn invalid_arg(arg: &str) {
	eprintln!("Invalid argument: {}", arg);
	eprintln!("Use -h or --help for usage information.");
}

fn inline_mode(interpreter: &mut Interpreter) {
	head();
	loop {
		print!("> ");
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
		let mut input = String::new();
		if std::io::stdin().read_line(&mut input).is_ok() {
			// run code
			println!("{}", input.trim());
		} else {
			break;
		}
	}
}

pub fn cli() {
	let args: Vec<String> = std::env::args().collect();

	let mut file_path = None;
	let mut debug = false;
	for arg in &args[1..] {
		if arg.starts_with('-') {
			if let Some(opt) = ARG_OPTIONS.iter().find(|opt| opt.short == arg || opt.long == arg) {
				match *opt {
					HELP_ARG_OPTION => {
						help();
						return;
					},
					VERSION_ARG_OPTION => {
						version();
						return;
					},
					DEBUG_ARG_OPTION => {
						debug = true;
					},
					_ => {
						invalid_arg(arg);
						return;
					},
				}
			} else {
				invalid_arg(arg);
				return;
			}
		} else if file_path.is_none() {
			file_path = Some(arg.clone());
		} else {
			eprintln!("Multiple file paths provided. Please provide only one.");
			return;
		}
	}

	let mut interpreter = if debug {
		Interpreter::debug()
	} else {
		Interpreter::new()
	};

	if file_path.is_none() {
		inline_mode(&mut interpreter);
	} else {
		// run file
	}
}
