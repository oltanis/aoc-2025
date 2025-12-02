use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn p1() -> Result<i16, Box<dyn std::error::Error>> {
    let mut password: i16 = 0;
    let mut position = 50;
    let file = File::open("../../data/d1/input")?;
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
        if position == 0 {
            password += 1;
        }
        //println!("[DEBUG] Line: {} Position: {}", line, pos);
    }
    Ok(password)
}

pub fn p2() -> Option<i16> {
    let mut password: i16 = 0;
    let mut position: i16 = 50;

    let file = File::open("../../data/d1/input").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let (direction, turn_amount) = line.split_at(1);
        let turn_amount = turn_amount.parse::<i16>().ok()?;
        for _ in 0..turn_amount {
            position = match direction {
                "R" => position - 1,
                "L" => position + 1,
                _ => return None,
            }
            .rem_euclid(100);
            if position == 0 {
                password += 1;
            }
        }
    }
    Some(password)
}

fn main() -> io::Result<()> {
    match p1() {
        Ok(pass) => println!("PASSWORD PART 1: {}", pass),
        Err(err) => eprintln!("Error: {}", err),
    }
    match p2() {
        Some(password) => println!("PASSWORD PART 2: {}", password),
        None => eprintln!("There was an error."),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1000() {
        let instructions = vec!["R1000", "L249", "L250", "L251"];
        let mut password = 0;
        let mut position = 50;
        for line in instructions {
            let inst = line.chars().next().unwrap();
            let num: i32 = line[1..].parse().unwrap();

            println!("Procesando: {} {}", inst, num);
            println!("  Posición inicial: {}", position);

            for _ in 0..num {
                match inst {
                    'R' => {
                        if position + 1 > 99 {
                            position = 0;
                        } else {
                            position += 1;
                        }
                    }
                    'L' => {
                        if position - 1 < 0 {
                            position = 99;
                        } else {
                            position -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
                if position == 0 {
                    password += 1;
                }
            }
            println!("Resultado final: {}", password);
            match line {
                "R1000" => assert_eq!(password, 10),
                "L249" => assert_eq!(password, 12),
                "L250" => assert_eq!(password, 15),
                "L251" => assert_eq!(password, 18),
                _ => println!("ERROR"),
            }
        }
    }

    #[test]
    fn test_r50() {
        let instructions = vec!["L50", "R50"];
        let mut password = 0;
        let mut position = 50;
        for line in instructions {
            let inst = line.chars().next().unwrap();
            let num: i32 = line[1..].parse().unwrap();

            println!("Procesando: {} {}", inst, num);
            println!("  Posición inicial: {}", position);

            for _ in 0..num {
                println!("POSICIÓN: {}", position);
                match inst {
                    'R' => {
                        if position + 1 > 99 {
                            position = 0;
                        } else {
                            position += 1;
                        }
                    }
                    'L' => {
                        if position - 1 < 0 {
                            position = 99;
                        } else {
                            position -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
                if position == 0 {
                    password += 1;
                }
            }
        }
        println!("Resultado final: {}", password);
        assert_eq!(password, 1);
    }
}
