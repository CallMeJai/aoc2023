// thanks to benjamin from uci rust club for this solution template
fn main() {
    let input = include_str!("../rsrc/input.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    0
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