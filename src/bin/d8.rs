use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn euclidean_distance((x1: i64, y1: i64, z1:i64), (x1: i64, y1: i64, z1:i64))


pub fn p1() -> Option<i64> {
    let file = File::open("../../data/d8/input-example").ok()?;
    let reader = BufReader::new(file);
    let mut coords: Vec<(i64, i64, i64)> = Vec::new();
    let lines = reader.lines();
    for line in lines {
        let line = line.ok()?;
        let mut coord = line.split(',');
        let (x, y, z) = (coord.next()?.parse::<i64>().unwrap(), coord.next()?.parse::<i64>().unwrap(), coord.next()?.parse::<i64>().unwrap());
        println!("COORD: x={x}, y={y}, z={z}");
        coords.push((x,y,z));
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
     *    match p2() {
     *        Some(ids) => println!("FRESH INGREDIENTS IDs: {}", ids),
     *        None => println!("THERE WAS AN ERROR"),
    }*/
}
