use std::fs;
use std::str;

const CASE_DIFFERENCE: i16 = 97 - 65; // constant difference between lowercase and uppercase letters in ASCII

fn part1(input: &str) -> String {
    let mut bytes: Vec<u8> = input.as_bytes().to_vec();
    let mut i = 0;
    // the below use of mutable vector is ultimately O(N^2) in worst case because each remove is O(N).
    // it's probably possible to do this in O(N) with 3 or 4 pointers, but it's making my brain hurt to think of it.
    while !bytes.is_empty() && i < bytes.len() - 1 {
        if (bytes[i] as i16 - bytes[i + 1] as i16).abs() == CASE_DIFFERENCE {
            // quick way to check if two letters are lowercase/uppercase versions of each other
            bytes.remove(i);
            bytes.remove(i); // i+1 is now i
            if i > 0 {
                i -= 1; // need to backtrack to handle possible chain reaction
            }
        } else {
            i += 1;
        }
    }
    str::from_utf8(&bytes).unwrap().to_string()
}

fn part2(input: &str) -> usize {
    let bytes: Vec<u8> = input.as_bytes().to_vec();
    let mut min: usize = part1(input).len();
    for c in 97..=122 {
        // 'a' thru 'z'
        let new_bytes: Vec<u8> = bytes
            .iter()
            .filter(|&ch| *ch != c && *ch != (c - CASE_DIFFERENCE as u8))
            .copied()
            .collect();
        if new_bytes.len() == bytes.len() {
            // if nothing was removed, no need to try reducing this one since we already reduced the full string above when initializing 'min'
            continue;
        }
        let new_str: &str = str::from_utf8(&new_bytes).unwrap();
        let reduced_str = part1(new_str);
        if reduced_str.len() < min {
            min = reduced_str.len();
        }
    }
    min
}

fn main() {
    let input =
        fs::read_to_string("input/day5.txt").expect("Something went wrong reading the file");
    println!("Day 5 Part 1: {}", part1(&input).len());
    println!("Day 5 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_empty_result() {
        assert_eq!("", part1("aA"));
        assert_eq!("", part1("Aa"));
        assert_eq!("", part1("abBA"));
        assert_eq!("", part1("aBbA"));
        assert_eq!("", part1("ABba"));
        assert_eq!("", part1("ABcCDdEeaAcCnNkKlopPbBcvVCOLba"));
    }

    #[test]
    fn test_part1_no_change() {
        assert_eq!("ABBa", part1("ABBa"));
        assert_eq!("Abba", part1("Abba"));
        assert_eq!("aabAAB", part1("aabAAB"));
    }

    #[test]
    fn test_part1_multi() {
        assert_eq!("dabCBAcaDA", part1("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2("dabAcCaCBAcCcaDA"));
    }
}
