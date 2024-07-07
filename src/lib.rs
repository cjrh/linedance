use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Closure = Box<dyn Fn() -> io::Result<Box<dyn BufRead>>>;

pub struct FileInput {
    readers: Vec<Closure>,
    current_reader_idx: usize,
    current_reader: Option<Box<dyn BufRead>>,
}

impl FileInput {
    pub fn new() -> io::Result<Self> {
        let readers = Self::parse_args()?;
        Ok(FileInput {
            readers,
            current_reader_idx: 0,
            current_reader: None,
        })
    }

    /// Build a list of readers from the command line arguments.
    /// If no files are provided, read from stdin.
    fn parse_args() -> io::Result<Vec<Closure>> {
        let mut readers = Vec::new();
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
                let arg = arg.to_string();
                let cb: Closure = Box::new(move || {
                    let file = File::open(&arg)?;
                    Ok(Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                });
                readers.push(cb);
            }
        }

        if readers.is_empty() {
            let cb: Closure = Box::new(|| {
                Ok(Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>)
            });
            readers.push(cb);
        }

        Ok(readers)
    }
}

impl Iterator for FileInput {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_reader_idx >= self.readers.len() {
                return None;
            }

            if self.current_reader.is_none() {
                match self.readers[self.current_reader_idx]() {
                    Ok(reader) => self.current_reader = Some(reader),
                    Err(e) => return Some(Err(e)),
                }
            }

            let mut line = String::new();
            match self.current_reader.as_mut().unwrap().read_line(&mut line) {
                Ok(0) => {
                    self.current_reader_idx += 1;
                    self.current_reader = None;
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
