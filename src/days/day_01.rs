use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn star_one() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_01.txt")?;
    let reader = BufReader::new(file);

    let mut combined_numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        if let Ok(digits) = line {
            let filtered_digits: String = digits.chars().filter(|c| c.is_digit(10)).collect();

            if let (Some(first), Some(last)) = (filtered_digits.chars().next(), filtered_digits.chars().last()) {
                if let (Some(first_digit), Some(last_digit)) = (first.to_digit(10), last.to_digit(10)) {
                    let combined = first_digit * 10 + last_digit;
                    combined_numbers.push(combined);
                }
            }
        }
    }

    println!("Answer: {:?}", combined_numbers.iter().sum::<u32>());

    Ok(())
}

pub fn star_two() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_01.txt")?;
    let reader = BufReader::new(file);

    let spelled_to_int_dict: Vec<(String, String)> = vec![
        ("zero".to_string(), "z0o".to_string()),
        ("one".to_string(), "o1e".to_string()),
        ("two".to_string(), "t2o".to_string()),
        ("three".to_string(), "t3e".to_string()),
        ("four".to_string(), "f4r".to_string()),
        ("five".to_string(), "f5e".to_string()),
        ("six".to_string(), "s6x".to_string()),
        ("seven".to_string(), "s7n".to_string()),
        ("eight".to_string(), "e8t".to_string()),
        ("nine".to_string(), "n9e".to_string()),
    ];

    let mut combined_numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        if let Ok(digits) = line {
            // check if the line contains any of the spelled out numbers and convert it to a digit
            let mut converted_digits: String = digits.clone();
            for (spelled, digit) in spelled_to_int_dict.iter() {
                converted_digits = converted_digits.replace(spelled, &digit.to_string());
            }
           
            let filtered_digits: String = converted_digits.chars().filter(|c| c.is_digit(10)).collect();

            if let (Some(first), Some(last)) = (filtered_digits.chars().next(), filtered_digits.chars().last()) {
                if let (Some(first_digit), Some(last_digit)) = (first.to_digit(10), last.to_digit(10)) {
                    let combined = first_digit * 10 + last_digit;
                    combined_numbers.push(combined);
                }
            }
        }
    }

    println!("Answer: {:?}", combined_numbers.iter().sum::<u32>());

    Ok(())
}
