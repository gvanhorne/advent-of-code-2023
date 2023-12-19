struct Game {
    id: u32,
    most_red: u32,
    most_green: u32,
    most_blue: u32,
}

fn build_game(game_info: &str) -> Game {
    let mut most_red: u32 = u32::MIN;
    let mut most_green: u32 = u32::MIN;
    let mut most_blue: u32 = u32::MIN;

    let mut game_iter = game_info.split(":");
    let id = game_iter.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
    let elf_reveals = game_iter.next().unwrap().split(";");
    for reveal in elf_reveals {
        let colour_groups = reveal.trim_start().split(",");
        for cubes in colour_groups {
            let mut parts = cubes.split_whitespace();

            let num_cubes: u32 = parts.next().unwrap().parse::<u32>().unwrap();
            let colour: &str = parts.next().unwrap();
            if colour == "red" {
                if num_cubes > most_red {
                    most_red = num_cubes;
                }
            } else if colour == "green" {
                if num_cubes > most_green {
                    most_green = num_cubes;
                }
            } else if colour == "blue" {
                if num_cubes > most_blue {
                    most_blue = num_cubes;
                }
            }
        }
    }
    Game {
        id,
        most_red,
        most_green,
        most_blue,
    }
}

fn process(input: &str) -> u32 {
    let output: u32 = input
    .lines()
    .filter(|&line| valid_game(line))
    .map(|line| {
        let game = build_game(line);
        // dbg!(game.id, game.min_blue, game.min_red, game.min_green);
        // println!("id: {}, red: {}, green: {}, blue: {}", game.id, game.most_red, game.most_green, game.most_blue);
        game.most_blue * game.most_red * game.most_green
    })
    .sum();

    output
}

fn valid_game(input: &str) -> bool {
    return true
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
        assert_eq!(result, 2286);
    }

    #[test]
    fn input2_works() {
        let file = include_str!("../../input2.txt");
        let result = process(file);
        assert_eq!(result, 68638);
    }
}
