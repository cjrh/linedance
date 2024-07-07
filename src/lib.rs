use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MakeReader = dyn Fn() -> io::Result<Box<dyn BufRead>>;

struct FileInput {
    // readers: Vec<Box<dyn Fn() -> io::Result<Box<dyn BufRead>>>>,
    readers: Vec<Box<MakeReader>>,
    current_reader_idx: usize,
    current_reader: Option<Box<dyn BufRead>>,
    buffer: String,
}

impl FileInput {
    pub fn new(parameter_name: &str) -> io::Result<Self> {
        let readers = Self::parse_args(parameter_name)?;
        Ok(FileInput {
            readers,
            current_reader_idx: 0,
            current_reader: None,
            buffer: String::new(),
        })
    }

    fn parse_args(parameter_name: &str) -> io::Result<Vec<Box<MakeReader>>> {
        let args: Vec<String> = env::args().collect();
        let file_args = Self::extract_file_args(parameter_name, &args);
        if file_args.is_empty() {
            Ok(vec![Box::new(|| Ok(Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>))])
        } else {
            Ok(file_args.into_iter().map(|arg| {
                Box::new(move || {
                    let file = File::open(&arg)?;
                    Ok(Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                }) as Box<dyn Fn() -> io::Result<Box<dyn BufRead>>>
            }).collect())
        }
    }

    fn extract_file_args(parameter_name: &str, args: &[String]) -> Vec<String> {
        let mut file_args = Vec::new();
        let mut reading_files = false;
        for arg in args.iter().skip(1) {
            if *arg == format!("--{parameter_name}") {
                reading_files = true;
            } else if reading_files && arg.starts_with('-') {
                break;
            } else if reading_files {
                file_args.push(arg.clone());
            }
        }
        file_args
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

            self.buffer.clear();
            let reader = match self.current_reader.as_mut() {
                Some(reader) => reader,
                None => return Some(Err(io::Error::new(io::ErrorKind::Other, "reader should be Some"))),
            };

            match reader.read_line(&mut self.buffer) {
                Ok(0) => {
                    self.current_reader_idx += 1;
                    self.current_reader = None;
                    continue;
                }
                Ok(_) => return Some(Ok(self.buffer.trim_end().to_string())),
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

pub fn input() -> io::Result<impl Iterator<Item = io::Result<String>>> {
    FileInput::new("files")
}
