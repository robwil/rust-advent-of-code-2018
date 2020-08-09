use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_line(line: &str) -> Claim {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    Claim {
        id: cap[1].parse::<usize>().unwrap(),
        left: cap[2].parse::<usize>().unwrap(),
        top: cap[3].parse::<usize>().unwrap(),
        width: cap[4].parse::<usize>().unwrap(),
        height: cap[5].parse::<usize>().unwrap(),
    }
}

type Grid = HashMap<(usize, usize), usize>;

fn part1(input: &str) -> (usize, Grid) {
    // "dumb" brute-force way to do this is to actually create the 1000x1000 grid and paint inside it
    // I found this nice snippet so will just keep it here for future reference:

    // let mut grid_raw = vec![0; grid_size * grid_size];
    // let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(grid_size).collect();
    // let grid = grid_base.as_mut_slice();
    // println!("grid = {:?}", grid);

    // more performant way is to just treat each visit as tuple and store that in a map
    // (I'm sure there might be a better mathy way, but don't feel like doing it that way)
    // let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut overlaps = 0;
    let mut visited = Grid::new();
    for line in input.lines() {
        let claim = parse_line(line);
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                // I purposely don't use visited.entry(..).or_insert(0) += 1 here
                // because I wanted to avoid the second iteration of the hashmap at end
                if visited.contains_key(&(x, y)) {
                    let current_val = *visited.get(&(x, y)).unwrap();
                    if current_val == 1 {
                        // only count overlap once
                        overlaps += 1;
                    }
                    visited.insert((x, y), current_val + 1);
                } else {
                    visited.insert((x, y), 1);
                }
            }
        }
    }
    (overlaps, visited)
}

fn part2(input: &str, grid: &Grid) -> usize {
    for line in input.lines() {
        let claim = parse_line(line);
        let mut has_overlaps = false;
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                if *grid.get(&(x, y)).unwrap() != 1 {
                    has_overlaps = true;
                    break;
                }
            }
        }
        if !has_overlaps {
            return claim.id;
        }
    }
    panic!("didn't find any claims without overlaps");
}

fn main() {
    let input =
        fs::read_to_string("input/day3.txt").expect("Something went wrong reading the file");
    let (overlapping_count, grid) = part1(&input);
    println!("Day 3 Part 1: {}", overlapping_count);
    println!("Day 3 Part 2: {}", part2(&input, &grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(4, part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2").0);
    }

    // #[test]
    // fn test_part1_full_file() {
    //     let input =
    //         fs::read_to_string("input/day3.txt").expect("Something went wrong reading the file");
    //     assert_eq!(115348, part1(&input));
    // }

    #[test]
    fn test_part2() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let (_, grid) = part1(&input);
        assert_eq!(3, part2(&input, &grid));
    }
}
