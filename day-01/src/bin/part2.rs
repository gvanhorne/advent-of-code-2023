use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let output: u32 = part1("./input2.txt");
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
    let calibration_values: Vec<u32> = concatenate_values(first_numbers, last_numbers);
    calibration_values.iter().sum()

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn concatenate_values(first_numbers: Vec<u32>, last_numbers: Vec<u32>) -> Vec<u32> {
    // Ensure that the input vectors have the same length
    if first_numbers.len() != last_numbers.len() {
        panic!("Input vectors must have the same length");
    }

    // Use zip to iterate over pairs of elements from both vectors
    let concatenated_values: Vec<u32> = first_numbers
        .into_iter()
        .zip(last_numbers.into_iter())
        .map(|(first, last)| {
            // Concatenate the numbers by multiplying the first by 10^d where d is the number of digits in the last
            let digits_in_last = (last as f64).log10().floor() as u32 + 1;
            first * 10u32.pow(digits_in_last) + last
        })
        .collect();

    concatenated_values
}

fn find_number_substrings_with_index(s: &str) -> Vec<(u32, usize)> {
    let mut result = Vec::new();

    for start in 0..s.len() {
        for end in start + 1..=s.len() {
            let substring = &s[start..end];
            if let Some(value) = word_to_digit(substring) {
                result.push((value, start));
            }
        }
    }

    result
}

fn find_min_index_tuple(numbers: &[(u32, usize)]) -> Option<(u32, usize)> {
    if let Some(&min_tuple) = numbers.iter().min_by_key(|&&(_, index)| index) {
        Some(min_tuple)
    } else {
        None
    }
}

fn find_max_index_tuple(numbers: &[(u32, usize)]) -> Option<(u32, usize)> {
    if let Some(&max_tuple) = numbers.iter().max_by_key(|&&(_, index)| index) {
        Some(max_tuple)
    } else {
        None
    }
}

fn extract_first_number(s: &str) -> Option<u32> {
    // Find the position of the first digit in the string
    let spelled_out_numbers = find_number_substrings_with_index(s);
    let min_spelled_number = find_min_index_tuple(&spelled_out_numbers);

    if let Some(index) = s.chars().position(|c| c.is_digit(10)) {
        // Check if there are spelled-out numbers with lower indices
        if let Some((min_value, min_index)) = min_spelled_number {
            return Some(if index < min_index { s.chars().nth(index).map(|c| c.to_digit(10)).flatten()? } else { min_value });
        } else {
            return s.chars().nth(index).map(|c| c.to_digit(10)).flatten();
        }
    } else {
        return min_spelled_number.map(|(min_value, _)| min_value);
    }
}

fn extract_last_number(s: &str) -> Option<u32> {
    // Find the position of the first digit in the string
    let spelled_out_numbers = find_number_substrings_with_index(s);
    let max_spelled_number = find_max_index_tuple(&spelled_out_numbers);
    let char_vec: Vec<char> = s.chars().collect();

    // Find the position of the last digit in the string
    if let Some(index) = char_vec.iter().rposition(|c| c.is_digit(10)) {
        // Check if there are spelled-out numbers with higher indices
        if let Some((max_value, max_index)) = max_spelled_number {
            return Some(if index > max_index {
                s.chars().nth(index).map(|c| c.to_digit(10)).flatten()?
            } else {
                max_value
            });
        } else {
            return s.chars().nth(index).map(|c| c.to_digit(10)).flatten();
        }
    } else {
        return max_spelled_number.map(|(max_value, _)| max_value);
    }
}


fn word_to_digit(word: &str) -> Option<u32> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input3_works() {
        let result: u32 = part1("./input3.txt");
        assert_eq!(result, 281);
    }

    #[test]
    fn word_to_digit_works() {
        let mut result: Option<u32> = word_to_digit("one");
        assert_eq!(result, Some(1));
        result = word_to_digit("two");
        assert_eq!(result, Some(2));
        result = word_to_digit("seventeen");
        assert_eq!(result, None);
        result = word_to_digit("nine");
        assert_eq!(result, Some(9));
    }
}
