use chrono::{NaiveDateTime, NaiveTime};
use clap::ArgGroup;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
	name = "stampy",
	about = "Tool to help you modify timestamps.",
	long_about = "Tool to help you modify timestamps.",
	after_long_help = 
	concat!(
		"\x1b[1;4mNotes:\x1b[0m",
		r#"
  - Providing the time is optional, the first hour of the day is selected by default unless using --preserve-time
  - Should be able to travel through symlinks (untested)
"#,
		"\x1b[1;4mExamples:\x1b[0m",
		r#"
  Can use the following format(s) for the date and time:
  - dd/mm/yy           ->  01/04/1987
  - dd/mm/yy hh:mm:ss  ->  '01/04/1987 22:00:00' (recommended. must be quoted though)
  - dd/mm/yy_hh:mm:ss  ->  01/04/1987_22:00:00
  - dd/mm/yyThh:mm:ss  ->  01/04/1987T22:00:00
"#),
	next_line_help = false
)]
#[command(
    group = ArgGroup::new("mode")
        .args(&["created", "modified", "accessed"])
        .required(true)
        .multiple(true)
)]
pub struct Args {
	#[arg(
        short = 'C',
        group = "mode",
        help = "Edit the 'created' stamp (UNSUPPORTED)",
        value_parser = parse_time,
        value_name = "TIME"
	)]
	pub created: Option<NaiveDateTime>,

	#[arg(
        short = 'M',
        group = "mode",
        help = "Edit the 'modified' stamp",
        value_parser = parse_time,
        value_name = "TIME",
	)]
	pub modified: Option<NaiveDateTime>,

	#[arg(
        short = 'A',
        group = "mode",
        help = "Edit the 'accessed' stamp",
        value_parser = parse_time,
        value_name = "TIME"
	)]
	pub accessed: Option<NaiveDateTime>,

	#[arg(help = "Prints every file that was modified", short = 'v', default_value = "true")]
	pub verbose: bool,
	
	#[arg(help = "The files to modify", short = 'r', default_value = "false")]
	pub recursive: bool,
	
	#[arg(help = "Carries over the hour/minute/seconds from the old file if a time wasn't provided", short = 'p', default_value = "false")]
	pub preserve_time: bool,

	#[arg(help = "The files to modify")]
	pub files: Vec<String>,
}

pub fn parse_time(arg: &str) -> Result<NaiveDateTime, String> {
	let date_format = "%d/%m/%Y";
	let time_format = "%H:%M:%S";
	let mut arg = arg.to_string();

	// Default time if only the date was specified
	if !arg.contains(':') {
		arg += " ";
		arg += NaiveTime::from_hms_opt(0, 0, 0).unwrap().format(time_format).to_string().as_str();
	}

	// Getting the date and time
	let mut last_error = None;
	for separator in &[' ', '_', 'T'] {
		let time = NaiveDateTime::parse_from_str(&arg, &format!("{date_format}{separator}{time_format}"));
		if let Ok(time) = time {
			return Ok(time);
		}
		last_error = Some(time.unwrap_err().to_string());
	}
	
	Err(last_error.unwrap())
}
