use regex::Regex;
use std::collections::HashMap;
use std::fs;

type GuardId = u32;
type Minute = u32;

fn parse_log(input: &str) -> HashMap<GuardId, HashMap<Minute, u32>> {
    let date_regex: &str = r"^\[(?P<date>\d{4}-\d{2}-\d{2}) \d{2}:(?P<minute>\d{2})\]";
    // [1518-05-19 23:50] Guard #2447 begins shift
    let begin_shift_regex: Regex = Regex::new(&format!(
        "{}{}",
        date_regex, r" Guard #(?P<guard_id>\d+) begins shift$"
    ))
    .unwrap();
    // [1518-10-11 00:33] falls asleep
    let falls_asleep_regex: Regex =
        Regex::new(&format!("{}{}", date_regex, r" falls asleep$")).unwrap();
    // [1518-09-26 00:18] wakes up
    let wakes_up_regex: Regex = Regex::new(&format!("{}{}", date_regex, r" wakes up$")).unwrap();

    // step 1: sort input for easier parsing
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    let mut record: HashMap<GuardId, HashMap<Minute, u32>> = HashMap::new();
    let mut guard_id: GuardId = 0;
    let mut sleep_minute: Minute = 0;
    for line in lines {
        // NB: This logic assumes valid input, that there is always sleep first/wake up second, after guard ID
        // step 2: parse each guard ID, then their awake/asleep times
        if let Some(cap) = begin_shift_regex.captures(line) {
            guard_id = cap["guard_id"].parse::<GuardId>().unwrap();
        } else if let Some(cap) = falls_asleep_regex.captures(line) {
            sleep_minute = cap["minute"].parse::<Minute>().unwrap();
        } else if let Some(cap) = wakes_up_regex.captures(line) {
            let wake_minute = cap["minute"].parse::<Minute>().unwrap();
            // step 3: add to global record of each guards sleep minutes HashMap<GuardId, HashMap<Minute, u32>>
            for minute in sleep_minute..wake_minute {
                *record
                    .entry(guard_id)
                    .or_insert_with(HashMap::new)
                    .entry(minute)
                    .or_insert(0) += 1;
            }
        }
    }
    record
}

fn part1(input: &str) -> (GuardId, Minute) {
    let record = parse_log(input);
    // step 4: find guard with most minutes asleep
    let sleepiest_guard: GuardId = record
        .iter()
        .map(|(guard_id, minute_counts)| (*guard_id, minute_counts.values().sum::<u32>()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    // step 5: get sleepiest minute for that guard
    let sleepiest_minute = *record
        .get(&sleepiest_guard)
        .unwrap()
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    println!(
        "guard {} slept the most overall, and they slept most at minute {}",
        sleepiest_guard, sleepiest_minute
    );
    (sleepiest_guard, sleepiest_minute)
}

fn part2(input: &str) -> (GuardId, Minute) {
    let record = parse_log(input);
    // step 4: find minute that was slept on the most
    let (guard_id, minute) = record
        .iter()
        .map(|(guard_id, minute_counts)| {
            (
                *guard_id,
                minute_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap(),
            )
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    println!(
        "{} slept {} times at minute {}",
        guard_id, minute.1, minute.0
    );
    (guard_id, *minute.0)
}

fn main() {
    let input =
        fs::read_to_string("input/day4.txt").expect("Something went wrong reading the file");
    let (sleepiest_guard, sleepiest_minute) = part1(&input);
    println!(
        "Day 4 Part 1: guard {} minute {} result {}",
        sleepiest_guard,
        sleepiest_minute,
        sleepiest_guard * sleepiest_minute
    );
    let (sleepiest_guard, sleepiest_minute) = part2(&input);
    println!(
        "Day 4 Part 2: guard {} minute {} result {}",
        sleepiest_guard,
        sleepiest_minute,
        sleepiest_guard * sleepiest_minute
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part1() {
        assert_eq!((10, 24), part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!((99, 45), part2(TEST_INPUT));
    }
}
