use dialoguer::Select;

mod days;

fn main() {    
    let items: Vec<(&str, fn() -> Result<_, std::io::Error>)> = vec![
        ("Day 1 - Star 1", days::day_01::star_one),
        ("Day 1 - Star 2", days::day_01::star_two),
        ("Day 2 - Star 1", days::day_02::star_one),
        ("Day 2 - Star 2", days::day_02::star_two),
        ("Day 3 - Star 1", days::day_03::star_one),
        ("Day 3 - Star 2", days::day_03::star_two),
    ];

    let selection = Select::new()
        .with_prompt("Select a challenge with the arrow buttons and press enter")
        .default(0)
        .items(&items.iter().map(|&(key, _)| key).collect::<Vec<_>>())
        .interact()
        .unwrap();
    
    let _ = items[selection].1();
}