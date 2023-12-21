// thanks to benjamin from uci rust club for this solution template
/*
 * turn S to O
 * iter 1:
 * turn . adjacent to O to X
 * turn adjacent Os to X
 * turn remaining Os to .
 * turn X to O
 * next iter
 * after iter 64, count O
 */
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
    assert_eq!(part1(input), 0);
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