use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [-r] [-i] [-o output.txt]", args[0]);
        process::exit(1);
    }

    let mut filename = "";
    let mut reverse = false;
    let mut case_insensitive = false;
    let mut output_file: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" => reverse = true, // Reverse sorting flag
            "-i" => case_insensitive = true, // Case-insensitive flag
            "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone()); // Output to file
                    i += 1;
                } else {
                    eprintln!("Error: Missing filename for -o option");
                    process::exit(1);
                }
            }
            _ => filename = &args[i], // File to read
        }
        i += 1;
    }

    if let Err(e) = sort_file(filename, reverse, case_insensitive, output_file) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn sort_file(filename: &str, reverse: bool, case_insensitive: bool, output_file: Option<String>) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader
    .lines()
    .map(|line| line.unwrap_or_else(|_| "[Invalid UTF-8]".to_string()))
    .collect();


    // Sort the lines
    if case_insensitive {
        lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    } else {
        lines.sort();
    }

    if reverse {
        lines.reverse();
    }

    if let Some(output) = output_file {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(output)?;
        for line in &lines {
            writeln!(file, "{}", line)?;
        }
    } else {
        for line in &lines {
            println!("{}", line);
        }
    }

    Ok(())
}