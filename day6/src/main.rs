use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: Point) -> usize {
        (self.x as i64 - other.x as i64).abs() as usize
            + (self.y as i64 - other.y as i64).abs() as usize
    }
}

struct BoundingBox {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

fn parse_line(line: &str) -> Point {
    let re = Regex::new(r"^(?P<x>\d+),\s+(?P<y>\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    Point {
        x: cap["x"].parse::<usize>().unwrap(),
        y: cap["y"].parse::<usize>().unwrap(),
    }
}

fn parse_input(input: &str) -> (Vec<Point>, BoundingBox) {
    let points: Vec<Point> = input.lines().map(|line| parse_line(line)).collect();
    let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);
    for point in &points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }
    (
        points,
        BoundingBox {
            min_x,
            max_x,
            min_y,
            max_y,
        },
    )
}

fn part1(input: &str) -> usize {
    // Basic thinking:
    // find min/max x and y and iterate through the range of all of them
    // for each point, calculate Manhattan distance to each of the input points
    // if there is exactly 1 min distance, attribute the current point to that input point (a running total in dictionary)
    // at the end, return max total from each of the input points

    let (points, bounding_box) = parse_input(input);

    // To help me debug, print grid
    // for x in 0 ..= max_x + 10 {
    //     for y in 0 ..= max_y + 10 {
    //         if point_totals.get(&Point{x,y}).is_some() {
    //             print!("A");
    //         } else {
    //             print!("_");
    //         }
    //     }
    //     print!("\n");
    // }

    let mut point_totals: HashMap<&Point, usize> = HashMap::new();
    let mut infinite_points: HashMap<&Point, bool> = HashMap::new();
    let mut min_points: Vec<&Point> = vec![];
    let mut min_distance;
    for x in bounding_box.min_x + 1..=bounding_box.max_x {
        for y in bounding_box.min_y + 1..=bounding_box.max_y {
            min_distance = usize::MAX;
            for point in &points {
                let distance = point.distance(Point { x, y });
                match distance.cmp(&min_distance) {
                    Ordering::Less => {
                        min_distance = distance;
                        min_points.clear();
                        min_points.push(point);
                    }
                    Ordering::Equal => min_points.push(point),
                    _ => (),
                }
            }
            if min_points.len() == 1 {
                let point = min_points.get(0).unwrap();
                *point_totals.entry(point).or_insert(0) += 1;
                // if a point "wins" anything with a min or max x/y in it, it will be infinite
                if x == bounding_box.min_x
                    || x == bounding_box.max_x
                    || y == bounding_box.min_y
                    || y == bounding_box.max_y
                {
                    infinite_points.insert(point, true);
                }
            }
        }
    }
    *point_totals
        .iter()
        .filter(|(k, _v)| infinite_points.get(*k).is_none())
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap()
}

fn part2(input: &str, max_distance: usize) -> usize {
    let (points, bounding_box) = parse_input(input);
    let mut total_area = 0;
    for x in bounding_box.min_x + 1..=bounding_box.max_x {
        for y in bounding_box.min_y + 1..=bounding_box.max_y {
            let mut total_distance = 0;
            for point in &points {
                total_distance += point.distance(Point { x, y });
            }
            if total_distance < max_distance {
                total_area += 1;
            }
        }
    }
    total_area
}

fn main() {
    let input =
        fs::read_to_string("input/day6.txt").expect("Something went wrong reading the file");
    println!("Day 6 Part 1: {}", part1(&input));
    println!("Day 6 Part 1: {}", part2(&input, 10000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!(17, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!(16, part2(input, 32));
    }
}
