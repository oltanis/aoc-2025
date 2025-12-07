use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_range(ranges: &mut Vec<(i64, i64)>, range: String) {
    let (a, b) = range.split_once('-').unwrap();
    let range_min = a.trim().parse::<i64>().unwrap();
    let range_max = b.trim().parse::<i64>().unwrap();
    //println!("RANGE: {}", range);
    ranges.push((range_min, range_max));
}

pub fn parse_range2(ranges: &mut Vec<(i64, i64)>, range: String) {
    let (a, b) = range.split_once('-').unwrap();
    let range_min = a.trim().parse::<i64>().unwrap();
    let range_max = b.trim().parse::<i64>().unwrap();
    let mut ranges_upt: Vec<(i64, i64)> = Vec::new();
    for &(existing_min, existing_max) in ranges.iter() {
        match (existing_min, existing_max) {
            (existing_min, existing_max) if range_min < existing_min => {
                ranges_upt.push((range_min, existing_max));
            },
            (existing_min, existing_max) if range_max > existing_max => {
                ranges_upt.push((existing_min, range_max));
            },
            (existing_min, existing_max) if range_min <= existing_min && range_max >= existing_max => {
                ranges_upt.push((range_min, range_max));
            },

            (existing_min, existing_max) if range_min <= existing_max && range_max >= existing_min => {
                let new_min = existing_min.min(range_min);
                let new_max = existing_max.max(range_max);
                ranges_upt.push((new_min, new_max));
            },
            _ => {}
        }
    }
    for (min, max) in ranges_upt{
        ranges.push((min, max));
    }
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

pub fn check_date_part2(ranges: &[(i64, i64)], ids: &mut Vec<i64>) -> Option<i64> {
    let mut fresh_ids = 0;
    for id in ids {
        for &(start, end) in ranges {
            if *id >= start && *id <= end {
                fresh_ids += (end - start) + 1;
            }
        }
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
    let file = File::open("../../data/d5/input-example").ok()?;
    let reader = BufReader::new(file);
    let mut change: bool = false;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();

    for range in reader.lines() {
        let range_str = range.ok()?.to_string();
        if !change {
            match range_str.as_str() {
                "" => change = true,
                _ => parse_range2(&mut ranges, range_str),
            }
        } else {
            println!("ID: {range_str}");
            ids.push(range_str.parse().unwrap());
        }
    }
    println!("check fresh ids...");
    check_date_part2(&mut ranges, &mut ids)
}

fn main() {
    println!("PART 1:");
    match p1() {
        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
        None => println!("THERE WAS AN ERROR"),
    }
    println!("PART 2:");
    match p1() {
        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
        None => println!("THERE WAS AN ERROR"),
    }
}
