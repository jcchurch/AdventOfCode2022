use std::io;
use std::io::prelude::*;

struct Position { x: i32, y: i32 }
struct Pair     { sensor: Position, beacon: Position }

#[derive(Clone)]
struct Range    { start:  i32, end: i32 }

fn read_pair(line: &String) -> Pair {
    let parts: Vec<&str> = line.split("=").collect();

    let part1: Vec<&str> = parts[1].split(",").collect();
    let sensor_x: i32 = part1[0].parse().unwrap();

    let part2: Vec<&str> = parts[2].split(":").collect();
    let sensor_y: i32 = part2[0].parse().unwrap();

    let part3: Vec<&str> = parts[3].split(",").collect();
    let beacon_x: i32 = part3[0].parse().unwrap();

    let beacon_y: i32 = parts[4].parse().unwrap();

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

fn compute_manhattan_distance(a: &Position, b: &Position) -> i32 {
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

fn get_range(a: &Pair, row: i32) -> Option<Range> {
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

fn find_ranges_at_row(pairs: &Vec<Pair>, row: i32) -> Vec<Range> {
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

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {

    let mut merged_ranges: Vec<Range> = Vec::new();     
    for range in ranges {
        let mut merged = false;
        for i in 0..merged_ranges.len() {
            match merge_two_ranges(range, &merged_ranges[i]) {
                Some(new_range) => {
                    merged_ranges[i] = new_range;
                    merged = true;
                    break;
                }
                None => {}
            }
        }
        if !merged {
            merged_ranges.push( Range { start: range.start, end: range.end } );
        }
    }

    merged_ranges
}

fn continious_merge(ranges: &Vec<Range>) -> Vec<Range> {
    let mut pass_a = merge_ranges(ranges);
    let mut pass_b = merge_ranges(&pass_a);

    while pass_a.len() != pass_b.len() {
        pass_a = pass_b.clone();
        pass_b = merge_ranges(&pass_b);
    }

    pass_b
}

fn compute_size_of_ranges(ranges: &Vec<Range>) -> i32 {
    let mut size: i32 = 0;
    for range in ranges {
        size += range.end - range.start  + 1;
    }

    size
}

fn is_value_in_range(ranges: &Vec<Range>, value: i32) -> bool {
    for range in ranges {
        if range.start <= value && value <= range.end {
            return true;
        }
    }

    false
}

fn is_beacon_seen(seen_beacons: &Vec<Position>, beacon: &Position) -> bool {
    for seen_beacon in seen_beacons {
        if seen_beacon.x == beacon.x && seen_beacon.y == beacon.y {
            return true;
        }
    }

    false
}

fn main() {
    let row = 2000000; 

    let pairs = read_all_pairs();
    let ranges = find_ranges_at_row(&pairs, row);
    let merged_ranges = continious_merge(&ranges);
    let mut size = compute_size_of_ranges(&merged_ranges);

    let mut seen_beacons: Vec<Position> = Vec::new();
    for pair in pairs {
        if pair.sensor.y == row && is_value_in_range(&merged_ranges, pair.sensor.x) && is_beacon_seen(&seen_beacons, &pair.sensor) == false {
            seen_beacons.push( Position { x: pair.sensor.x, y: pair.sensor.y } );
            size -= 1;
        }
        if pair.beacon.y == row && is_value_in_range(&merged_ranges, pair.beacon.x) && is_beacon_seen(&seen_beacons, &pair.beacon) == false {
            seen_beacons.push( Position { x: pair.beacon.x, y: pair.beacon.y } );
            size -= 1;
        }
    }

    println!("{}", size);
}
