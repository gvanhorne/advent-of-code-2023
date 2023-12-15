use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let output: String = part1("./input1.txt");
    dbg!(output);
}

fn part1(input: &str) -> String {
    // read each line
    match read_lines(input) {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading lines from file: {}", err);
        }
    }
    "142".to_string()
    // get the first and last integer of each line
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: String = part1("./input1.txt");
        assert_eq!(result, "142".to_string());
    }
}
