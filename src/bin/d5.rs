use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn add_range(ranges: &mut Vec<(i64, i64)>, new: (i64, i64)) {
    let (mut a, mut b) = new;
    let mut out = Vec::new();

    for &(x, y) in ranges.iter() {
        if b < x || a > y {
            out.push((x, y));
        } else {
            a = a.min(x);
            b = b.max(y);
        }
    }

    out.push((a, b));
    out.sort_by_key(|r| r.0);
    *ranges = out;
}

pub fn parse_range(ranges: &mut Vec<(i64, i64)>, range: String) {
    let (a, b) = range.split_once('-').unwrap();
    let range_min = a.trim().parse::<i64>().unwrap();
    let range_max = b.trim().parse::<i64>().unwrap();
    add_range(ranges, (range_min, range_max));
}

pub fn check_date(ranges: &[(i64, i64)], ids: &mut Vec<i64>) -> Option<i64> {
    let mut fresh_ids = 0;
    for id in ids {
        for &(start, end) in ranges {
            if *id >= start && *id <= end {
                fresh_ids += 1;
                break;
            }
        }
    }
    Some(fresh_ids)
}

pub fn check_ranges(ranges: &[(i64, i64)]) -> Option<i64> {
    let mut fresh_ids = 0;
    for (min, max) in ranges {
        fresh_ids += max - min + 1;
    }
    Some(fresh_ids)
}

pub fn p1() -> Option<i64> {
    let file = File::open("../../data/d5/input").ok()?;
    let reader = BufReader::new(file);
    let mut change: bool = false;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();

    for range in reader.lines() {
        let range_str = range.ok()?.to_string();
        if !change {
            match range_str.as_str() {
                "" => change = true,
                _ => parse_range(&mut ranges, range_str),
            }
        } else {
            //println!("ID: {range_str}");
            ids.push(range_str.parse().unwrap());
        }
    }
    println!("check fresh ids...");
    check_date(&mut ranges, &mut ids)
}

pub fn p2() -> Option<i64> {
    let file = File::open("../../data/d5/input").ok()?;
    let reader = BufReader::new(file);
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for range in reader.lines() {
        let range_str = range.ok()?.to_string();
        match range_str.as_str() {
            "" => break,
            _ => parse_range(&mut ranges, range_str),
        }
    }
    println!("check fresh ids...");
    check_ranges(&mut ranges)
}

fn main() {
    println!("PART 1:");
    match p1() {
        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
        None => println!("THERE WAS AN ERROR"),
    }
    println!("PART 2:");
    match p2() {
        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
        None => println!("THERE WAS AN ERROR"),
    }
}
