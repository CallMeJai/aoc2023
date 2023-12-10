use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./rsrc/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum: i32 = 0;
        for line in lines {
            
            if let Ok(inp) = line {
                let input = inp.replace("zero", "z0o")
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");
                let mut first: i32 = -1;
                let mut last: i32 = -1;
                for c in input.chars() {
                    match c {
                        '0' if first < 0 => first = 0,
                        '0' if first >= 0 => last = 0,
                        '1' if first < 0 => first = 1,
                        '1' if first >= 0 => last = 1,
                        '2' if first < 0 => first = 2,
                        '2' if first >= 0 => last = 2,
                        '3' if first < 0 => first = 3,
                        '3' if first >= 0 => last = 3,
                        '4' if first < 0 => first = 4,
                        '4' if first >= 0 => last = 4,
                        '5' if first < 0 => first = 5,
                        '5' if first >= 0 => last = 5,
                        '6' if first < 0 => first = 6,
                        '6' if first >= 0 => last = 6,
                        '7' if first < 0 => first = 7,
                        '7' if first >= 0 => last = 7,
                        '8' if first < 0 => first = 8,
                        '8' if first >= 0 => last = 8,
                        '9' if first < 0 => first = 9,
                        '9' if first >= 0 => last = 9,
                        _ => (),
                    }
                }
                sum += 10 * first + last;
                if last == -1 {
                    sum += 1 + first;
                }
            }
        }
        println!("Sum is {}", sum);
    } else {
        println!("File not found.");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}