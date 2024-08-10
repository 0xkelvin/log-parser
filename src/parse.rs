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

        Ok(())
    }
}
