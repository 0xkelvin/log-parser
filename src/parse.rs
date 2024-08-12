use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;

pub struct Parse;

impl Parse {
    pub fn parse_booting(
        source_log_file: Option<PathBuf>,
        destination_files: Option<PathBuf>,
    ) -> io::Result<()> {
        println!("xxxx {:#?} and {:#?}", source_log_file, destination_files);
        // Ensure the output directory exists
        let output_directory = if let Some(dir) = destination_files {
            if dir.is_dir() {
                dir
            } else {
                eprintln!("Error: {} is not a directory", dir.display());
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: No output directory provided");
            std::process::exit(1);
        };
        // Open the file in read-only mode and create a buffered reader
        if let Some(input_path) = source_log_file {
            let file = File::open(&input_path)?;
            let reader = BufReader::new(file);

            // The string we are searching for
            let search_string = "Booting Linux on physical CPU 0x0000000000";

            let mut mark: Vec<usize> = Vec::new();
            let mut lines: Vec<String> = Vec::new();
            // Regular expression to capture the date and time
            let re = Regex::new(r"([A-Za-z]{3} \d{2} \d{2}:\d{2}:\d{2})").unwrap();
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
                // println!("{}", &lines[mark[i] + 1]);
                let timestamp = if let Some(captures) = re.captures(&lines[mark[i] + 1]) {
                    if let Some(data_time) = captures.get(0) {
                        let date_time_str = data_time.as_str();
                        let format_data_time = date_time_str.replace(" ", "_");
                        format!("{}", format_data_time)
                    } else {
                        format!("data_time_not_found")
                    }
                } else {
                    format!("not_match_found")
                };
                let file_name = format!("{}_num#{}.txt", timestamp, i + 1);
                let output_path = output_directory.join(file_name);
                let mut output_file = File::create(&output_path)?;
                println!("{} to {}", mark[i], mark[i + 1]);
                for line in &lines[mark[i]..mark[i + 1]] {
                    writeln!(output_file, "{}", line)?;
                }
            }
        }

        Ok(())
    }
}
