#![feature(let_chains)]
use std::{fmt::{Display, Error, Formatter}, collections::HashSet};
// thanks to benjamin from uci rust club for this solution template
fn main() {
    let input = include_str!("../rsrc/input.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

#[derive(Hash)]
struct Num {
    value: usize,
    digit_count: usize,
    validity: bool,
    start: (usize, usize),
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if !self.validity {
            write!(f, "in");
        }
        write!(f, "valid part number: {} starting at ({}, {})", self.value, self.start.1, self.start.0)
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Eq for Num {}

fn string_slice_to_2d_array(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn is_number(c: &char) -> bool {
    &'0' <= c && c <= &'9'
}

fn is_symbol(c: &char) -> bool {
    c != &'.' && !is_number(c)
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn traverse(dir: Direction, pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char>{
    match dir {
        Direction::Left => check_left(pos, engine),
        Direction::Right => check_right(pos, engine),
        Direction::Up => check_up(pos, engine),
        Direction::Down => check_down(pos, engine),
        Direction::UpLeft => check_up_left(pos, engine),
        Direction::UpRight => check_up_right(pos, engine),
        Direction::DownLeft => check_down_left(pos, engine),
        Direction::DownRight => check_down_right(pos, engine),
    }
}

fn check_left(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.1 == 0 {
        None
    } else {
        Some(&engine[pos.0][pos.1 - 1])
    }
}

fn check_right(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.1 >= engine[0].len() - 1 { // rectangular matrix idc
        None
    } else {
        Some(&engine[pos.0][pos.1 + 1])
    }
}

fn check_up(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 == 0 {
        None
    } else {
        Some(&engine[pos.0 - 1][pos.1])
    }
}

fn check_down(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 >= engine.len() - 1 {
        None
    } else {
        Some(&engine[pos.0 + 1][pos.1])
    }
}

fn check_up_left(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 == 0 || pos.1 == 0 {
        None
    } else {
        Some(&engine[pos.0 - 1][pos.1 - 1])
    }
}

fn check_up_right(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 == 0 || pos.1 >= engine[0].len() - 1 {
        None
    } else {
        Some(&engine[pos.0 - 1][pos.1 + 1])
    }
}

fn check_down_left(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 == engine.len() - 1 || pos.1 == 0 {
        None
    } else {
        Some(&engine[pos.0 + 1][pos.1 - 1])
    }
}

fn check_down_right(pos: (usize, usize), engine: &Vec<Vec<char>>) -> Option<&char> {
    if pos.0 >= engine.len() - 1 || pos.1 >= engine[0].len() - 1 {
        None
    } else {
        Some(&engine[pos.0 + 1][pos.1 + 1])
    }
}

fn get_num(pos: (&usize, &usize), engine: &Vec<Vec<char>>) -> Num {
    println!("called at pos: {:?}", pos);
    let mut num: Num = Num {value: engine[*pos.0][*pos.1].to_digit(10).expect(&format!("{} at position ({}, {}) is not a number", engine[*pos.0][*pos.1], *pos.0, *pos.1)) as usize, digit_count: 1, validity: false, start: (*pos.0, *pos.1)};
    while let Some(c) = traverse(Direction::Left, num.start, engine)
        && is_number(c) {
            println!("number {} to the left of {:?}!", c, num.start);
            num.start.1 -= 1;
            num.value = c.to_digit(10).unwrap() as usize;
    }
    while let Some(c) = traverse(Direction::Right, (num.start.0, num.start.1 + num.digit_count - 1), engine)
        && is_number(c) {
            println!("number {} to the right of ({}, {})!", c, num.start.0, num.start.1 + num.digit_count - 1);
            num.value *= 10;
            num.value += c.to_digit(10).unwrap() as usize;
            num.digit_count += 1;
        }
    num
}

fn part1(input: &str) -> usize {
    let mut part_numbers: HashSet<Num> = Default::default();
    let engine: Vec<Vec<char>> = string_slice_to_2d_array(input);
    for y in 0..engine.len() {
        for x in 0..engine[0].len() {
            if is_symbol(&engine[y][x]) {
                if let Some(c) = traverse(Direction::Left, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&y, &(x - 1)), &engine));
                }
                if let Some(c) = traverse(Direction::Right, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&y, &(x + 1)), &engine));
                }
                if let Some(c) = traverse(Direction::Up, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y - 1), &x), &engine));
                }
                if let Some(c) = traverse(Direction::Down, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y + 1), &x), &engine));
                }
                if let Some(c) = traverse(Direction::UpLeft, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y - 1), &(x - 1)), &engine));
                }
                if let Some(c) = traverse(Direction::UpRight, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y - 1), &(x + 1)), &engine));
                }
                if let Some(c) = traverse(Direction::DownLeft, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y + 1), &(x - 1)), &engine));
                }
                if let Some(c) = traverse(Direction::DownRight, (y, x), &engine) && is_number(c) {
                    part_numbers.insert(get_num((&(y + 1), &(x + 1)), &engine));
                }
            }
        }
    }
    part_numbers.iter().map(|a| a.value).sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part1(input), 4361);
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part2(input), 0);
}