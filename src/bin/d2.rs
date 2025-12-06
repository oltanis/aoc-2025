use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn digit_checker(num: i128) -> bool{
    let num_str = num.to_string();
    match num_str.chars().all(|c| c == num_str.chars().next().unwrap()){
        true => return true,
        false => return false,
    }
}


pub fn p1() -> Option<i128> {
    let file = File::open("../../data/d2/input-example").ok()?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    let mut ids = 0;
    
    reader.read_line(&mut input).ok()?;
    println!("INPUT: {}", input);
    let ranges = input.split(",");

    for range in ranges {
        let range_str = range.to_string();
        println!("RANGE: {}", range_str);
        let (a, b) = range.split_once('-')?;
        let input1 = a.trim().parse::<i128>().ok()?;
        let input2 = b.trim().parse::<i128>().ok()?;

        println!("INPUT1: {}", input1);
        println!("INPUT2: {}\n\n", input2);

        for num in input1..input2 + 1{
            match digit_checker(num.try_into().unwrap()){
                true => {
                    println!("ID {num} IS NOT A VALID ID. ADDING");
                    ids += num;
                },
                false => continue,
            }
        }
    }
    Some(ids)
}

fn main(){
    match p1() {
        Some(ids) => println!("IDs PART 1: {}", ids),
        None => println!("There was an error"),
    }
}
