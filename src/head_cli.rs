use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::i64;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

type HeadrResult = Result<(), Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    name = "headr",
    version = "0.1.0",
    about = "output the first part of files",
    long_about = "Print the first 10 lines of each FILE to standard output.  With more than one FILE, precede each with a header giving the file name."
)]
pub struct HeadCli {
    #[arg(
        name = "lines",
        long = "lines",
        short = 'n',
        default_value_t = 10,
        help = "print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file"
    )]
    number_of_lines: i64,
    #[arg(
        name = "bytes",
        long = "bytes",
        short = 'c',
        help = "print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file"
    )]
    number_of_bytes: Option<i64>,
    #[arg(
        name = "files",
        help = "Path to files : with no FILE, or when FILE is -, read standard input"
    )]
    files: Vec<PathBuf>,
}

impl HeadCli {
    pub fn run(&self) -> HeadrResult {
        match self.files.is_empty() {
            true => self.print_from_stdin()?,
            false => self.print_from_file()?,
        };

        Ok(())
    }

    fn print_from_file(&self) -> HeadrResult {
        let mut result: String = String::new();
        for file in &self.files {
            if self.files.len() > 1 {
                let title = match result.is_empty() {
                    true => format!("==> {} <==\n", file.to_string_lossy()),
                    false => format!("\n==> {} <==\n", file.to_string_lossy()),
                };

                result.push_str(title.as_str());
            }
            let of = File::open(file).expect("Error opening file");
            let buf = BufReader::new(of);

            let limit: i64 = if self.number_of_lines.is_negative() {
                let count = BufReader::new(File::open(file).expect("Failed to count line"))
                    .lines()
                    .count() as i64;
                count - self.number_of_lines.abs()
            } else {
                self.number_of_lines
            };

            for (k, line) in buf.lines().enumerate() {
                if k as i64 >= limit {
                    break;
                }

                if let Ok(l) = line {
                    result.push_str(format!("{}\n", l).as_str());
                }
            }
        }

        print!("{}", result);

        Ok(())
    }

    fn print_from_stdin(&self) -> HeadrResult {
        let mut lines = BufReader::new(std::io::stdin()).lines();
        let mut counter = 0;
        loop {
            if counter >= self.number_of_lines.abs() {
                break;
            }

            if let Some(Ok(line)) = lines.next() {
                println!("{}", line);
                counter += 1;
            }
        }

        Ok(())
    }
}
