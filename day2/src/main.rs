use std::collections::HashMap;
use std::fs;

// Part 1 I did using a simple hashmap and iterative approach.
// BurntSushi's solution takes advantage of the fact that letter counts can use an array of 256 ints instead of a full fledged HashMap. 
// They also use iter().any() which simplifies logic a bit.
fn part1(input: &str) -> u64 {
    let mut double_letter_count: u64 = 0;
    let mut triple_letter_count: u64 = 0;
    for line in input.lines() {
        let mut letter_counts: HashMap<char, u64> = HashMap::new();
        for c in line.chars() {
            match letter_counts.get(&c) {
                Some(&current_val) => letter_counts.insert(c, current_val + 1),
                None => letter_counts.insert(c, 1),
            };
        }
        let mut has_double_letter = false;
        let mut has_triple_letter = false;
        for (_, count) in letter_counts.iter() {
            if *count == 2 {
                has_double_letter = true;
            } else if *count == 3 {
                has_triple_letter = true;
            }
        }
        if has_double_letter {
            double_letter_count += 1;
        }
        if has_triple_letter {
            triple_letter_count += 1;
        }
    }
    double_letter_count * triple_letter_count
}

// Part 2 asks to find two lines that differ by only 1 character.
// The most straight-forward way I could think to do this is a standard O(N^2) loop
// comparing each line with others. BurntSushi does the same but uses some more functional logic.
fn part2(input: &str) -> String {
    for (i, line1) in input.lines().enumerate() {
        for (j, line2) in input.lines().enumerate() {
            if i == j {
                continue;
            }
            let line1_chars = line1.as_bytes();
            let line2_chars = line2.as_bytes();
            // note: all lines have same length, so short-cutting this logic
            let mut diff = 0;
            let mut overlapping_chars = String::new();
            for k in 0..line1_chars.len() {
                if line1_chars[k] != line2_chars[k] {
                    diff += 1;
                    if diff > 1 {
                        break;
                    }
                } else {
                    overlapping_chars.push(line1_chars[k] as char)
                }
            }
            if diff == 1 {
                return overlapping_chars;
            }
        }
    }
    panic!("could not find expected string")
}

fn main() {
    let input =
        fs::read_to_string("input/day2.txt").expect("Something went wrong reading the file");
    println!("Day 1 Part 1: {}", part1(&input));
    println!("Day 1 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(12, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!("fgij", part2(input));
    }
}
