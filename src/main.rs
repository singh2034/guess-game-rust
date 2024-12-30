use rand::prelude::*;
use std::io;

fn main() {
    let guess_list: [&str; 3] = ["grapes", "banana", "apple"];
    let mut rng: ThreadRng = thread_rng();

    let index: usize = rng.gen_range(0..guess_list.len());
    let random_fruit: &str = guess_list[index];
    // println!("Random Fruit : {}", random_fruit);
    let mut input: String = String::new();

    loop {
        input.clear();
        println!("Guess the fruit");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected: String = input.trim().to_lowercase();
                // println!("Fruit Selected : {}", fruit_selected);
                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Well Well Well. What a loser, can't even guess a guess well.");
                    continue;
                }
                if guess_checker(&fruit_selected, random_fruit) {
                    println!("Why cheer so much, it's just a guess! You Won!");
                    break;
                } else {
                    println!("Retry");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    return fruit_selected == random_fruit;
}
