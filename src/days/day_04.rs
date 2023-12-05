use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

pub fn star_one() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_04.txt")?;
    let reader = BufReader::new(file);

    let mut scores: Vec<i32> = Vec::new();

    for line in reader.lines() {        
        if let Ok(line) = line {

            let line_split: Vec<&str> = line.split(':').collect();
            let card_split: Vec<&str> = line_split[1].split('|').collect();
            let winning_numbers: Vec<&str> = card_split[0].trim().split_whitespace().collect();
            let card_numbers: Vec<&str> = card_split[1].trim().split_whitespace().collect();
            
            println!("Card: {:?}", card_numbers);
            println!("Winning Numbers: {:?}", winning_numbers);

            let mut points = 1;
            let mut at_least_one_win = false;
            for number in card_numbers {
                if winning_numbers.contains(&number) {
                    println!("Found a match: {}", number);

                    if at_least_one_win {
                        points *= 2; 
                    }

                    at_least_one_win = true;  
                }
            }
            if at_least_one_win {
                scores.push(points);
            }        
        }
    }

    println!("Scores: {:?}", scores);
    let sum: i32 = scores.iter().sum(); 
    println!("Answer: {}", sum);

    Ok(())
}

pub fn star_two() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_04.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();

    let matches: Vec<_> = lines
        .iter()
        .map(|line| map_card(line))
        .collect();

    let mut total_cards: HashMap<usize, usize> = (0..lines.len()).map(|i| (i, 1)).collect();

    for (i, matches) in matches.iter().enumerate() {
        let c_amount = *total_cards.get(&i).unwrap();
        for index in i + 1..i + matches + 1 {
            if let Some(amount) = total_cards.get_mut(&(index)) {
                *amount += c_amount;
            }
        }
    }
  
    let sum = total_cards.values().sum::<usize>().to_string();
    println!("Answer: {}", sum);

    Ok(())
}


fn map_card(card: &str) -> usize {
    let line_split: Vec<_> = card.split(": ").collect();
    let id_split: Vec<&str> = line_split[0].split_whitespace().collect();
    let card_id = id_split[1].trim();

    let card_split: Vec<_> = line_split[1].split(" | ").collect();
    let winning_numbers: HashSet<_> = card_split[0].trim().split_whitespace().collect();
    let card_numbers: Vec<_> = card_split[1].trim().split_whitespace().collect();

    card_numbers
        .iter()
        .fold(0, |acc: usize, &number| match winning_numbers.contains(&number) {
            true => acc + 1,
            false => acc,
        })
}