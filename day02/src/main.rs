use std::cmp::max;

// thanks to benjamin from uci rust club for this solution template
fn main() {
    let input = include_str!("../rsrc/input.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

struct Game {
    red: usize,
    green: usize,
    blue: usize, 
}

fn parse_game(game: &str) -> Game {
    let mut parsed_game: Game = Game {red: 0, green: 0, blue: 0};
    for pick in game.split(", ") {
        let color: Vec<_> = pick.split(' ').collect();
        if color[1] == "red" {
            parsed_game.red = color[0].parse::<usize>().unwrap();
        } else if color[1] == "green" {
            parsed_game.green = color[0].parse::<usize>().unwrap();
        } else if color[1] == "blue" {
            parsed_game.blue = color[0].parse::<usize>().unwrap();
        }
    }
    parsed_game
}

fn part1(input: &str) -> usize {
    let start: Game = Game {red: 12, green: 13, blue: 14};
    let mut game_counter = 0;
    let mut sum = 0;
    for line in input.split('\n') {
        let mut valid_game: bool = true;
        game_counter += 1;
        let games: Vec<_> = line.split(": ").collect();
        for game in games[1].split("; ") {
            let current_move = parse_game(game);
            if start.red < current_move.red ||
                start.green < current_move.green ||
                start.blue < current_move.blue {
                    valid_game = false;
                    break;
                }
        }
        if valid_game {
            sum += game_counter
        }
    }
    sum   
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part1(input), 8);
}


fn game_power(games: &str) -> usize {
    let mut min_set: Game = Game {red: 0, green: 0, blue: 0};
    for game in games.split("; ") {
        let current_move = parse_game(game);
        min_set.red = max(current_move.red, min_set.red);
        min_set.green = max(current_move.green, min_set.green);
        min_set.blue = max(current_move.blue, min_set.blue);

    }
    power(&min_set)
}

fn power(set: &Game) -> usize {
    set.red * set.blue * set.green
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        let games: Vec<_> = line.split(": ").collect();
        sum += game_power(games[1]);
    }
    sum
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part2(input), 2286);
}