use std::collections::HashSet;
use std::fs;

// Part 1 is simple and straight forward, just summing the inputs (treating them as signed ints)
fn part1(input: &str) -> i64 {
    input.lines().map(|line| line.parse::<i64>().unwrap()).sum()
}

// Part 2 we need to find repeated frequency.
// here's me doing in the straight-forward iterative way
fn part2(input: &str) -> i64 {
    let mut already_seen: HashSet<i64> = HashSet::new();
    let mut current_sum: i64 = 0;
    loop {
        for line in input.lines() {
            current_sum += line.parse::<i64>().unwrap();
            if already_seen.contains(&current_sum) {
                return current_sum;
            }
            already_seen.insert(current_sum);
        }
    }
}

fn main() {
    let input =
        fs::read_to_string("input/day1.txt").expect("Something went wrong reading the file");
    println!("Day 1 Part 1: {}", part1(&input));
    println!("Day 1 Part 2: {}", part2(&input));
}
