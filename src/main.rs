use std::fmt;
use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct CountResult {
    filename: String,
    char_count: u64,
    line_count: u64,
    word_count: u64,
}

impl CountResult {
    fn new(filename: &str) -> CountResult {
        CountResult {
            filename: filename.to_string(),
            char_count: 0,
            line_count: 0,
            word_count: 0,
        }
    }

    fn increment_by(&mut self, other: &Self) {
        self.char_count += other.char_count;
        self.line_count += other.line_count;
        self.word_count += other.word_count;
    }
}

impl fmt::Display for CountResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n\tchar count: {}\n\tline count: {}\n\tword_count: {}\n", self.filename, self.char_count, self.line_count, self.word_count)
    }
}

#[derive(Debug)]
enum CountError {
    NoFilenameSpecifed,
    IOError(String)
}

fn count_file(filename: &str) -> Result<CountResult, Box<error::Error>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = CountResult::new(&filename);

    loop {
        let len = reader.read_line(&mut line)?;

        match len {
            _len if len > 0 => {
                // increment the line count
                result.line_count += 1;
                // increment character and word counts (where appropriate)
                for c in line.chars() {
                    match c {
                        _ if c.is_whitespace() => result.word_count += 1,
                        _ => result.char_count += 1,
                    }
                }
            },
            _ => break,
        }
        line.clear();
    }

    Ok(result)
}

fn run_count() -> Result<CountResult, CountError> {
    let args: Vec<String> = env::args().collect();
    let mut total_counts = CountResult::new("Totals");

    if args.len() <= 1 {
        return Err(CountError::NoFilenameSpecifed)
    }
     
    for filename in args.iter().skip(1) {
        let result = count_file(&filename);

        match result {
            Ok(res) => {
                total_counts.increment_by(&res);
                println!("{}", res);
            },
            Err(err) => return Err(CountError::IOError(err.to_string())),
        }
    }
    return Ok(total_counts)
}

fn main() {
    match run_count() {
        Ok(result) => {
            println!("{}\n{}", "-".repeat(25), result);
        }
        Err(err) => {
            match err {
                CountError::NoFilenameSpecifed => println!("ERROR: No filename specified"),
                _ => panic!("Unknown error occurred: {:?}", err)
            }
        }
    }    
}
