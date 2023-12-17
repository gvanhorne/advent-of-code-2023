struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn build_game(game_info: &str) -> Game {
    let mut game_iter = game_info.split(":");
    let id = game_iter.next().map(|f| f.split(" ").nth(1)).unwrap().unwrap().parse::<u32>().unwrap();
    let mut reveals_iter = game_iter.next().unwrap().split(";");
    let first_game = reveals_iter.next();
    let mut first_game_iter = first_game.unwrap().trim_start().split(" ");
    let num_cubes = first_game_iter.next().unwrap().parse::<u32>();
    let cube_colour = first_game_iter.next().unwrap().trim.parse::<String>();
    dbg!(cube_colour);
    // let cube_colour = first_game.unwrap().split(" ").next();
    // let cube_colour = first_game.unwrap().next();
    // reveals.unwrap().split(",");
    Game {
        id,
        red: 2,
        green: 3,
        blue: 4,
    }
}

fn process(input: &str) -> u32 {
    let output: u32 = input.lines().filter(|&line| !invalid_game(line)).flat_map(|line| {
        line.chars().skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .ok()
    }).sum();

    output
}

fn invalid_game(input: &str) -> bool {
    build_game(input);
    return true
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
