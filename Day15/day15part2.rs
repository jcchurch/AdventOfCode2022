use std::io;
use std::io::prelude::*;

struct Position { x: i64, y: i64 }
struct Pair     { sensor: Position, beacon: Position }

#[derive(Clone)]
struct Range    { start:  i64, end: i64 }

fn read_pair(line: &String) -> Pair {
    let parts: Vec<&str> = line.split("=").collect();

    let part1: Vec<&str> = parts[1].split(",").collect();
    let sensor_x: i64 = part1[0].parse().unwrap();

    let part2: Vec<&str> = parts[2].split(":").collect();
    let sensor_y: i64 = part2[0].parse().unwrap();

    let part3: Vec<&str> = parts[3].split(",").collect();
    let beacon_x: i64 = part3[0].parse().unwrap();

    let beacon_y: i64 = parts[4].parse().unwrap();

    Pair { sensor: Position { x: sensor_x, y: sensor_y },
           beacon: Position { x: beacon_x, y: beacon_y } }
}

fn read_all_pairs() -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        pairs.push( read_pair( &line.unwrap() ) );
    }

    pairs
}

fn compute_manhattan_distance(a: &Position, b: &Position) -> i64 {
    let mut first = a.x - b.x;
    let mut second = a.y - b.y;

    if first < 0 {
        first *= -1;
    }

    if second < 0 {
        second *= -1;
    }

    first + second
}

fn get_range(a: &Pair, row: i64) -> Option<Range> {
    let mut distance = compute_manhattan_distance(&a.sensor, &a.beacon);

    let mut offset = a.sensor.y - row;
    if offset < 0 {
        offset *= -1;
    }

    distance -= offset;
    if distance < 0 {
        return None;
    }

    Some(Range { start: a.sensor.x - distance, end: a.sensor.x + distance })
}

fn find_ranges_at_row(pairs: &Vec<Pair>, row: i64) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();

    for pair in pairs {
        match get_range(&pair, row) {
            Some(range) => ranges.push( range ),
            None => {}
        }
    }

    ranges
}

fn merge_two_ranges(a: &Range, b: &Range) -> Option<Range> {
    let mut first = a;
    let mut second = b;

    if b.start < a.start {
        let temp = first;
        first = second;
        second = temp;
    }

    if first.end + 1 >= second.start {
        let mut end = first.end;
        if second.end > end {
            end = second.end;
        }

        return Some(Range {start: first.start, end: end});
    }

    None
}

fn merge_ranges(ranges: &mut Vec<Range>) -> Vec<Range> {
    ranges.sort_by( |a, b| a.start.cmp(&b.start) );

    let mut merged_ranges: Vec<Range> = Vec::new();     
    merged_ranges.push( Range { start: ranges[0].start, end: ranges[0].end } );

    for i in 1..ranges.len() {
        let range = merged_ranges.pop().unwrap();
        match merge_two_ranges(&range, &ranges[i]) {
            Some(new_range) => merged_ranges.push(new_range),
            None => {
                merged_ranges.push(range);
                merged_ranges.push( Range { start: ranges[i].start, end: ranges[i].end } );
            }
        }
    }

    merged_ranges
}

fn main() {
    let max: i64 = 4000000;
    let pairs = read_all_pairs();

    for row in 0..=max {
        let mut ranges = find_ranges_at_row(&pairs, row);
        let merged_ranges = merge_ranges(&mut ranges);

        if merged_ranges.len() > 1 {
            for range in merged_ranges {
                if range.end + 1 <= max {
                    println!("{} * 4000000 + {} = {}", range.end + 1, row, (range.end + 1) * max + row);
                }
            }
        }
    }
}
