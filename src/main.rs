mod cli;
mod parse;
use crate::cli::Cli;
use crate::cli::Commands;
use crate::parse::Parse;
use clap::{Command, Parser, Subcommand};
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    // println!("{:#?}", cli);
    match &cli.command {
        Some(Commands::HmiBooting {
            source_path,
            dest_path,
        }) => Parse::parse_booting(source_path.clone(), dest_path.clone()),

        None => Ok({}),
    }
}
// use std::env;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write};
// use std::path::Path;

// fn main() -> io::Result<()> {
//     // Collect command-line arguments
//     let args: Vec<String> = env::args().collect();

//     // Check if the required arguments provided
//     if args.len() < 3 {
//         eprintln!("Usage: {} <input_file> <output_directory>", args[0]);
//         std::process::exit(1);
//     }

//     // The path to the input file
//     let input_path = Path::new(&args[1]);

//     // The path to the output directory
//     let output_directory = Path::new(&args[2]);

//     // Ensure the output directory exists
//     if !output_directory.is_dir() {
//         eprintln!("Error: {} is not a directory", output_directory.display());
//         std::process::exit(1);
//     }
//     // Open the file in read-only mode and create a buffered reader
//     let file = File::open(&input_path)?;
//     let reader = BufReader::new(file);

//     // The string we are searching for
//     let search_string = "Booting Linux on physical CPU 0x0000000000";

//     let mut mark: Vec<usize> = Vec::new();
//     let mut lines: Vec<String> = Vec::new();
//     mark.push(0);
//     // Iterate through lines in the file
//     for (current_index, line_result) in reader.lines().enumerate() {
//         let line = line_result?;
//         if line.contains(search_string) {
//             mark.push(current_index - 1);
//             // println!("Found '{}' on line {}", search_string, current_index + 1);
//         }
//         lines.push(line);
//     }
//     mark.push(lines.len());

//     // Iterate over the ranges in the mark vector
//     for i in 0..(mark.len() - 1) {
//         let file_name = format!("startup_num#:_{}.txt", i + 1);
//         let output_path = output_directory.join(file_name);
//         let mut output_file = File::create(&output_path)?;
//         println!("{} to {}", mark[i], mark[i + 1]);
//         for line in &lines[mark[i]..mark[i + 1]] {
//             writeln!(output_file, "{}", line)?;
//         }
//     }

//     Ok(())
// }
