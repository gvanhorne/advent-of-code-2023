use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let output: u32 = part1("./input1.txt");
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut first_numbers: Vec<u32> = Vec::new();
    let mut last_numbers: Vec<u32> = Vec::new();

    // read each line
    match read_lines(input) {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    if let Some(first_number) = extract_first_number(&ip) {
                        // Add the first number to the array
                        first_numbers.push(first_number);
                    };
                    if let Some(last_number) = extract_last_number(&ip) {
                        last_numbers.push(last_number);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading lines from file: {}", err);
        }
    }
    // Return the sum of the array of first numbers
    last_numbers.iter().sum()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to extract the first number from a string
fn extract_first_number(s: &str) -> Option<u32> {
    // Find the position of the first digit in the string
    if let Some(index) = s.chars().position(|c| c.is_digit(10)) {
        // Try to parse the substring as a u32
        s.chars().nth(index).map(|c| c.to_digit(10)).flatten()
    } else {
        None // No digit found in the string
    }
}

fn extract_last_number(s: &str) -> Option<u32> {
    let char_vec: Vec<char> = s.chars().collect();
    // Find the position of the last digit in the string
    if let Some(index) = char_vec.iter().rposition(|c| c.is_digit(10)) {
        // Try to parse the substring as a u32
        s.chars().nth(index).map(|c| c.to_digit(10)).flatten()
    } else {
        None // No digit found in the string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: u32 = part1("./input1.txt");
        assert_eq!(result, 12);
    }
}
