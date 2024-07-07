use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileInput {
    // Using closures to lazily open files
    readers: Vec<Box<dyn Fn() -> io::Result<Box<dyn BufRead>>>>,
    current_reader: usize,
    current_r: Box<dyn BufRead>
}

impl FileInput {
    pub fn new() -> io::Result<Self> {
        let mut readers: Vec<Box<dyn Fn() -> io::Result<Box<dyn BufRead>>>> = Vec::new();
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
                readers.push(Box::new(move || {
                    let file = File::open(&arg)?;
                    Ok(Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                }));
            }
        }

        if readers.is_empty() {
            readers.push(Box::new(|| Ok(Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>)));
        }

        let first_reader = readers[0]()?;

        Ok(FileInput {
            readers,
            current_reader: 0,
            current_r: first_reader,
        })
    }
}

impl Iterator for FileInput {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_reader >= self.readers.len() {
            return None;
        }

        let mut line = String::new();
        match self.current_r.read_line(&mut line) {
            Ok(0) => {
                self.current_reader += 1;
                if self.current_reader >= self.readers.len() {
                    None
                } else {
                    self.current_r = self.readers[self.current_reader]().unwrap();
                    self.next()
                }
            },
            Ok(_) => return Some(Ok(line.trim_end().to_string())),
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn input() -> io::Result<impl Iterator<Item = io::Result<String>>> {
    FileInput::new()
}
