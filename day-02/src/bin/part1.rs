fn process(input: &str) -> u32 {
    let output: u32 = input.lines().flat_map(|line| {
        line.chars().skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .ok()
    }).sum();

    output
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1_works() {
        let file = include_str!("../../input1.txt");
        let result = process(file);
        assert_eq!(result, 8);
    }
}
