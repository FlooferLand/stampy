use crate::args::Args;
use chrono::DateTime;
use clap::Parser;
use filetime::{set_file_atime, set_file_mtime, FileTime};
use std::fs::read_dir;

mod args;
mod log;

fn main() {
    let args = args::Args::parse();

    if args.created.is_some() {
        fatal_error!(-1, "Setting the creation date is planned but currently unsupported, see https://github.com/FlooferLand/stampy/blob/main/README.md#todo");
    }
    
    // No flags
    if args.created.is_none() && args.modified.is_none() && args.accessed.is_none() {
        fatal_error!(-1, "You must specify an operation!\nCheck '--help'");
    }
    
    // Directory stuff
    let mut counter = 0;
    for entry in &args.files {
        run_operation(&args, &mut counter, entry);
    }
    
    // "Modified X files" text
    println!(
        "{counter} {FileWas} modified.",
        FileWas = if counter == 1 { "file was" } else { "files were" }
    );
    
    // Exit
    std::process::exit(0);
}

#[derive(Debug)]
pub enum OperationType {
    Created,
    Modified,
    Accessed
}

pub fn run_operation(args: &Args, counter: &mut i32, file: &str) {
    if !std::fs::exists(file).unwrap_or(false) {
        error!("File '{file}' does not exist!");
        return;
    }
    let Ok(metadata) = std::fs::metadata(file) else {
        error!("Skipped '{file}'. Unable to gather metadata");
        return;
    };

    // TODO: Set the directory node time as well
    if metadata.is_dir() {
        match read_dir(file) {
            Ok(readdir) => {
                for entry in readdir {
                    match entry {
                        Ok(entry) => {
                             run_operation(args, counter, &*entry.path().to_string_lossy());
                        }
                        Err(err) => error!("Unable to read directory '{file}': {err}"),
                    }
                }
            }
            Err(err) => error!("Unable to read directory '{file}': {err}"),
        }
    } else if metadata.is_file() {
        if let Some(time) = &args.modified {
            let time = FileTime::from_unix_time(time.and_utc().timestamp_millis() / 1000, 0);
            if let Err(err) = set_file_mtime(file, time) {
                error!("Failed to set the 'modified' time for '{file}' (time={Time}): {err}", Time = DateTime::from_timestamp(time.unix_seconds(), time.nanoseconds()).unwrap_or_default());
                return;
            }
        }
        if let Some(time) = &args.accessed {
            let time = FileTime::from_unix_time(time.and_utc().timestamp_millis() / 1000, 0);
            if let Err(err) = set_file_atime(file, time) {
                error!("Failed to set the 'accessed' time for '{file}' (time={Time}): {err}", Time = DateTime::from_timestamp(time.unix_seconds(), time.nanoseconds()).unwrap_or_default());
                return;
            }
        }
        *counter += 1;
    }
}
