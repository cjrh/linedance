use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileInput {
    readers: Vec<Box<dyn BufRead>>,
    current_reader: usize,
}

impl FileInput {
    pub fn new() -> io::Result<Self> {
        let mut readers: Vec<Box<dyn BufRead>> = Vec::new();
        let args: Vec<String> = env::args().collect();
        let mut reading_files = false;

        for arg in args.iter().skip(1) {
            if arg == "--files" {
                reading_files = true;
                continue;
            }

            if reading_files && arg.starts_with('-') {
                break;
            }

            if reading_files {
                let file = File::open(arg)?;
                readers.push(Box::new(BufReader::new(file)));
            }
        }

        if readers.is_empty() {
            // If no files were specified, read from stdin
            readers.push(Box::new(BufReader::new(io::stdin())));
        }

        Ok(FileInput {
            readers,
            current_reader: 0,
        })
    }
}

impl Iterator for FileInput {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_reader >= self.readers.len() {
                return None;
            }

            let mut line = String::new();
            match self.readers[self.current_reader].read_line(&mut line) {
                Ok(0) => {
                    // End of current file, move to next reader
                    self.current_reader += 1;
                    continue;
                }
                Ok(_) => return Some(Ok(line.trim_end().to_string())),
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

pub fn input() -> io::Result<impl Iterator<Item = io::Result<String>>> {
    FileInput::new()
}
