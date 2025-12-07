use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn add() -> Option<i128> {}

pub fn p1() -> Option<i64> {
    let file = File::open("../../data/d6/input-example").ok()?;
    let reader = BufReader::new(file);
    let mut elems: Vec<(i64, i64)> = Vec::new();
    let lines = reader.lines();

    for line in lines {
        let mut i = 1;
        let line_str = line.unwrap();
        for token in line_str.split_whitespace() {
            let value = token.trim().parse::<i64>().unwrap();
            println!("category: {} | value: {}", i, value);
            elems.push((i, value));
            i += 1;
        }
    }
    Some(0)
}

fn main() {
    println!("PART 1:");
    match p1() {
        Some(ops) => println!("RESULT OF THE OPERATION: {}", ops),
        None => println!("THERE WAS AN ERROR"),
    }
    /*println!("PART 2:");
    match p2() {
        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
        None => println!("THERE WAS AN ERROR"),
    }*/
}
