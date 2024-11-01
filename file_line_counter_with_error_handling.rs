use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_lines(filename: &str) -> Result<usize, io::Error> {
    let archive = File::open(filename)?;

    let reader = io::BufReader::new(archive);

    let number_of_lines = reader.lines().count();
    Ok(number_of_lines)
}

fn main () {
    let filename = "my_archive.txt";

    match count_lines(filename) {
        Ok(count) => println!("The number of lines is {}", count),
        Err(e) => eprintln!("Error to read the file: {}", e),
    }
}
