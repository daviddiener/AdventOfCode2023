use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn star_one() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_01.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {        
        if let Ok(line) = line {
        
        }
    }

    Ok(())
}

pub fn star_two() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_01.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {        
        if let Ok(line) = line {
        
        }
    }

    Ok(())
}