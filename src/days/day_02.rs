use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn star_one() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_02.txt")?;
    let reader = BufReader::new(file);

    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    let mut possible_game_ids: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(game) = line {
            
            let info: Vec<&str> = game.split(':').collect();

            let game_part: Vec<&str> = info[0].split_whitespace().collect();
            let game_id = game_part[1].parse::<i32>().unwrap();
            let mut game_possible: bool = true;
            
            let parts: Vec<&str> = info[1].split(';').map(|s| s.trim()).collect();

            for part in parts {
                let cubes: Vec<&str> = part.split(',').map(|s| s.trim()).collect();

                for cube in cubes {
                    let cube_info: Vec<&str> = cube.split_whitespace().collect();

                    if cube_info[1] == "red" && cube_info[0].parse::<i32>().unwrap() > max_red_cubes {
                        println!("Detected impossible cube amount: {:?} with GameId: {}", cube_info, game_id);
                        game_possible = false;
                        break;
                    }

                    if cube_info[1] == "green" && cube_info[0].parse::<i32>().unwrap() > max_green_cubes {
                        println!("Detected impossible cube amount: {:?} with GameId: {}", cube_info, game_id);
                        game_possible = false;
                        break;
                    }

                    if cube_info[1] == "blue" && cube_info[0].parse::<i32>().unwrap() > max_blue_cubes {
                        println!("Detected impossible cube amount: {:?} with GameId: {}", cube_info, game_id);
                        game_possible = false;
                        break;
                    }
                }
            }

            if game_possible {
                possible_game_ids.push(game_id);
            }

        }
    }

    println!("Possible Game Ids: {:?}", possible_game_ids);
    let sum: i32 = possible_game_ids.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}

pub fn star_two() -> Result<(), std::io::Error> {
    let file = File::open("src/inputs/day_02.txt")?;
    let reader = BufReader::new(file);

    let mut power_of_sets: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(game) = line {

            let mut max_red_cube_amount = 0;
            let mut max_green_cube_amount = 0;
            let mut max_blue_cube_amount = 0;
            
            let info: Vec<&str> = game.split(':').collect();

            let game_part: Vec<&str> = info[0].split_whitespace().collect();
            let game_id = game_part[1].parse::<i32>().unwrap();
            
            let parts: Vec<&str> = info[1].split(';').map(|s| s.trim()).collect();

            for part in parts {
                let cubes: Vec<&str> = part.split(',').map(|s| s.trim()).collect();

                for cube in cubes {
                    let cube_info: Vec<&str> = cube.split_whitespace().collect();

                    println!("Cube Info: {:?}", cube_info);

                    if cube_info[1] == "red" && cube_info[0].parse::<i32>().unwrap() > max_red_cube_amount {
                        println!("Detected new max red cube amount: {}", cube_info[0]);
                        max_red_cube_amount = cube_info[0].parse::<i32>().unwrap();
                        continue;
                    }

                    if cube_info[1] == "green" && cube_info[0].parse::<i32>().unwrap() > max_green_cube_amount {
                        println!("Detected new max green cube amount: {}", cube_info[0]);
                        max_green_cube_amount = cube_info[0].parse::<i32>().unwrap();
                        continue;
                    }

                    if cube_info[1] == "blue" && cube_info[0].parse::<i32>().unwrap() > max_blue_cube_amount {
                        println!("Detected new max blue cube amount: {}", cube_info[0]);
                        max_blue_cube_amount = cube_info[0].parse::<i32>().unwrap();
                        continue;
                    }
                }
            }

            println!("Game: {}, Max Red: {}, Max Green: {}, Max Blue: {}", game_id, max_red_cube_amount, max_green_cube_amount, max_blue_cube_amount);
            power_of_sets.push(max_red_cube_amount * max_green_cube_amount * max_blue_cube_amount);
        }
    }

    let sum: i32 = power_of_sets.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}