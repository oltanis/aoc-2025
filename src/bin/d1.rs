use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn p1() -> Result<i16, Box<dyn std::error::Error>>{
    let mut password: i16 = 0;
    let mut position = 50;
    let file = File::open("../../data/d1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let inst = line.chars().next();
        let num: i16 = line[1..].parse()?;

        match inst {
            Some('R') => position = (position + num) % 100,
            Some('L') => position = ((position - num) % 100).rem_euclid(100),
            _ => eprintln!("Instrucción inválida: {}", inst.unwrap()),
        }
        if position == 0{
            password += 1;
        }
        //println!("[DEBUG] Line: {} Position: {}", line, pos);
    }
    Ok(password)
}


pub fn p2() -> Result<i32, Box<dyn std::error::Error>> {
    let mut password: i32 = 0;
    let mut position: i32 = 50;
    let file = File::open("../../data/d1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let inst = line.chars().next().unwrap();
        let num: i32 = line[1..].parse()?;

        match inst {
            'R' => {
                // hits durante la rotación
                password += (position + num) / 100;
                position = (position + num) % 100;
            }
            'L' => {
                let old_pos = position;
                position = (position - num).rem_euclid(100);

                // calcular cruces por cero hacia la izquierda
                let hits = (num - old_pos + 99) / 100;
                password += hits;
            }
            _ => unreachable!(),
        }

        if position == 0 {
            password += 1; // cuenta si termina en 0
        }
    }

    Ok(password)
}

pub fn p2_brutus()-> Result<i16, Box<dyn std::error::Error>>{
    let mut password: i16 = 0;
    let mut position = 50;
    let file = File::open("../../data/d1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let inst = line.chars().next();
        let num: i16 = line[1..].parse()?;

        if inst == Some('R') {
            for _ in 0..num {
                position += 1;
                if position > 99 {
                    position = 0;
                }
                if position == 0 {
                    password += 1;
                }
            }
        } else if inst == Some('L') {
            for _ in 0..num {
                position -= 1;
                if position < 0 {
                    position = 99;
                }
                if position == 0 {
                    password += 1;
                }
            }
        } else {
            return Err(format!("Invalid instruction: {:?}", inst).into());
        }
    }

    Ok(password)
}

fn main() -> io::Result<()> {
    match p1(){
        Ok(pass) => println!("PASSWORD 1: {}", pass),
        Err(err) => eprintln!("Error: {}", err),
    }
    match p2(){
        Ok(pass) => println!("PASSWORD 2: {}", pass),
        Err(err) => eprintln!("Error: {}", err),
    }
    match p2_brutus(){
        Ok(pass) => println!("PASSWORD 2 BRUTUS: {}", pass),
        Err(err) => eprintln!("Error: {}", err),
    }
    Ok(())
}
