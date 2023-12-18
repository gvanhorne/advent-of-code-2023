struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn build_game(game_info: &str) -> Game {
    let mut max_red: u32 = u32::MIN;
    let mut max_green: u32 = u32::MIN;
    let mut max_blue: u32 = u32::MIN;

    let mut game_iter = game_info.split(":");
    let id = game_iter.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
    let elf_reveals = game_iter.next().unwrap().split(";");
    for reveal in elf_reveals {
        let colour_groups = reveal.trim_start().split(",");
        for cubes in colour_groups {
            let mut parts = cubes.split_whitespace();

            let num_cubes: u32 = parts.next().unwrap().parse::<u32>().unwrap();
            let colour: &str = parts.next().unwrap();
            if (colour == "red") && (num_cubes > max_red) {
                max_red = num_cubes;
            } else if (colour == "green") && (num_cubes > max_green) {
                max_green = num_cubes;
            } else if (colour == "blue") && (num_cubes > max_blue) {
                max_blue = num_cubes;
            }
        }
    }
    Game {
        id,
        max_red,
        max_green,
        max_blue,
    }
}

fn process(input: &str) -> u32 {
    let output: u32 = input.lines().filter(|&line| !valid_game(line)).flat_map(|line| {
        line.chars().skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .ok()
    }).sum();

    output
}

fn valid_game(input: &str) -> bool {
    let red_cubes: u32 = 12;
    let green_cubes: u32 = 13;
    let blue_cubes: u32 = 14;
    let game = build_game(input);
    return game.max_blue > blue_cubes || game.max_green > green_cubes || game.max_red > red_cubes;
}

fn main() {
    let file = include_str!("../../input2.txt");
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

    #[test]
    fn input2_works() {
        let file = include_str!("../../input2.txt");
        let result = process(file);
        assert_eq!(result, 2776);
    }
}
