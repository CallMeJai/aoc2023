use std::collections::HashSet;
fn main() {
    let input = include_str!("../rsrc/input.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for card in input.lines() {
        let (w, m) = card.split_once(':').unwrap().1.split_once('|').unwrap();
        let mut winning: HashSet<usize> = HashSet::new();
        let mut mine: HashSet<usize> = HashSet::new();
        for x in w.split(' ') {
            if let Ok(num) = x.parse::<usize>() {
                winning.insert(num);
            }
        }
        for x in m.split(' ') {
            if let Ok(num) = x.parse::<usize>() {
                mine.insert(num);
            }
        }
        sum += (1 << (winning.intersection(&mine).count())) >> 1;
    }
    sum
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part1(input), 13);
}

fn part2(input: &str) -> usize {
    let mut cards = vec![1; input.lines().count()];
    let mut current_card = 0;
    for card in input.lines() {
        let (w, m) = card.split_once(':').unwrap().1.split_once('|').unwrap();
        let mut winning: HashSet<usize> = HashSet::new();
        let mut mine: HashSet<usize> = HashSet::new();
        for x in w.split(' ') {
            if let Ok(num) = x.parse::<usize>() {
                winning.insert(num);
            }
        }
        for x in m.split(' ') {
            if let Ok(num) = x.parse::<usize>() {
                mine.insert(num);
            }
        }
        for i in 1..winning.intersection(&mine).count() + 1 {
            cards[current_card + i] += cards[current_card] 
        }
        current_card += 1;
    }
    cards.iter().sum()

}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = include_str!("../rsrc/test.txt");
    assert_eq!(part2(input), 30);
}