use rand::Rng;
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

        let guess_number = guess.trim().parse::<i32>().expect("could not parse to i32");

        if correct_number == guess_number {
            println!("Congratulations, you guessed the number!");
            game_state = "off";
        } else {
            let msg = if correct_number > guess_number {
                "That's too low"
            } else {
                "That's too high"
            };
            println!("{}", msg);
        }
    }
}
