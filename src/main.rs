use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Collect command-line arguments

    // The path to the input file
    let path = Path::new("/Users/kelvin/Downloads/aug06_083000_150000.log.txt");

    // Open the file in read-only mode and create a buffered reader
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // The string we are searching for
    let search_string = "Booting Linux on physical CPU 0x0000000000";

    let mut mark: Vec<usize> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    mark.push(0);
    // Iterate through lines in the file
    for (current_index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if line.contains(search_string) {
            mark.push(current_index - 1);
            // println!("Found '{}' on line {}", search_string, current_index + 1);
        }
        lines.push(line);
    }
    mark.push(lines.len());

    // Iterate over the ranges in the mark vector
    for i in 0..(mark.len() - 1) {
        let file_name = format!("startup_num#:_{}.txt", i + 1);
        let output_path = Path::new(&file_name);
        let mut output_file = File::create(&output_path)?;
        println!("{} to {}", mark[i], mark[i + 1]);
        for line in &lines[mark[i]..mark[i + 1]] {
            writeln!(output_file, "{}", line)?;
        }
    }

    Ok(())
}
