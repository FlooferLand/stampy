use clap::Parser;
use filetime::{set_file_atime, set_file_mtime, FileTime};
use crate::args::Args;

mod args;
mod log;

fn main() {
    let args = args::Args::parse();

    if args.created.is_some() {
        fatal_error!(-1, "Setting the creation date is planned but currently unsupported, see https://github.com/FlooferLand/stampy/README.md");
    }
    
    // No flags
    if args.created.is_none() && args.modified.is_none() && args.accessed.is_none() {
        fatal_error!(-1, "You must specify an operation!\nCheck '--help'");
    }
    
    // Directory stuff
    let mut counter = 0;
    for entry in &args.files {
        if let Err(err) = run_operation(&args, &mut counter, entry) {
            error!("{err}");
        }
    }
    
    // "Modified X files" text
    println!("{counter} files were modified.");
    
    // Exit
    std::process::exit(0);
}

#[derive(Debug)]
pub enum OperationType {
    Created,
    Modified,
    Accessed
}

pub fn run_operation(args: &Args, counter: &mut i32, file: &str) -> Result<(), String> {
    if !std::fs::exists(file).unwrap_or(false) {
        return string_error!("'{file}' does not exist!", !args.verbose);
    }
    let Ok(metadata) = std::fs::metadata(file) else {
        return string_error!("Skipped '{file}'. Unable to gather metadata", !args.verbose);
    };

    // TODO: Set the directory times as well
    if metadata.is_dir() {
        if let Err(err) = run_operation(args, counter, file) {
            return Err(err);
        }
    } else if metadata.is_file() {
        if let Some(time) = &args.modified {
            if let Err(err) = set_file_mtime(file, FileTime::from_unix_time(time.and_utc().timestamp_millis() / 1000, time.and_utc().timestamp_micros() as u32)) {
                return Err(err.to_string());
            }
        }
        if let Some(time) = &args.accessed {
            if let Err(err) = set_file_atime(file, FileTime::from_unix_time(time.and_utc().timestamp_millis() / 1000, time.and_utc().timestamp_micros() as u32)) {
                return Err(err.to_string());
            }
        }
    }

    *counter += 1;
    Ok(())
}
