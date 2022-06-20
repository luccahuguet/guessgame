use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Program initializing...");
    println!("Lets play a game");
    start();
}

fn start() {
    let mut game_state = "on";
    let correct_number;

    correct_number = rand::thread_rng().gen_range(1..101);
    while game_state == "on" {
        println!("Type a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("could not read input");

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&correct_number) {
            Ordering::Less => println!("That's too low"),
            Ordering::Greater => println!("That's too high"),
            Ordering::Equal => {
                println!("Congratulations, you guessed the number!");
                game_state = "off";
            }
        }
    }
}
