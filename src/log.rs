#[macro_export]
macro_rules! info {
    ($($text:tt)*) => {{
	    use colored::Colorize;
        println!(
		    "{Mark} {Text}",
	        Mark = "Info:".bold().underline().white(),
	        Text = format!($($text)*)
        );
    }};
}

#[macro_export]
macro_rules! error {
    ($($text:tt)*) => {{
	    use colored::Colorize;
        println!(
		    "{Mark} {Text}",
	        Mark = "Error:".bold().underline().red(),
	        Text = format!($($text)*)
        );
    }};
}

#[macro_export]
macro_rules! fatal_error {
    ($exit_code:literal, $($text:tt)*) => {{
	    use colored::Colorize;
        println!(
		    "{Mark} {Text}",
	        Mark = "Error:".bold().underline().red(),
	        Text = format!($($text)*)
        );
        std::process::exit($exit_code);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($text:tt)*) => {{
	    use colored::Colorize;
        println!(
		    "{Mark} {Text}",
	        Mark = "Warning:".bold().underline().yellow(),
	        Text = format!($($text)*)
        );
    }};
}

#[macro_export]
macro_rules! debug {
    ($($text:tt)*) => {{
	    #[cfg(debug_assertions)] {
		    use colored::Colorize;
	        println!(
			    "{Mark} {Text}",
		        Mark = "Debug:".bold().underline().bright_black(),
		        Text = format!($($text)*)
	        );
	    }
    }};
}
