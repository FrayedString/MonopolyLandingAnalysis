use std::{io, process};
use std::io::*;

use futures::executor::block_on;

mod game_simulation;


fn main() {
    println!("Monopoly space landing simulator.  Enter 'q' at any input to quit.");
        println!("");

        let (num_players, num_turns) = get_user_input();
            

        let work = game_simulation::run_simulation(num_players, num_turns);
        block_on(work);


        println!("");
        println!("Press Enter to quit");

        let mut buf: [u8; 1] = [0; 1];

        let _quit = std::io::stdin().read(&mut buf);
}



fn get_user_input() -> (u32, u32) {
    fn check_quit_input(input: &str) {
        if input == "q" || input == "Q" {
            println!("Thanks for playing!  Goodbye!");
            process::exit(0);
        }
    }

    loop {
        let mut user_input = String::new();
        println!("Enter number of players (2-8): ");

        if let Err(e) = io::stdin().read_line(&mut user_input) {
            println!("Error reading input, please try again: {}", e);
            println!("");
            continue;
        }

        let input_num_players = user_input.trim();

        check_quit_input(input_num_players);

        let num_players: u32 =
            match input_num_players.trim().parse() {
                Ok(num) if num < 2 => {
                    println!("Number of players must be >= 2");
                    println!("");
                    continue;
                },
                Ok(num) if num > 8 => {
                    println!("Number of players must be <= 8");
                    println!("");
                    continue;
                },
                Ok(num) => num,
                Err(msg) => {
                    println!("Invalid number of players: {}", msg);
                    println!("");
                    continue;
                }
            };


        user_input.clear();
        
        println!("Enter number of turns (1-500): ");

        if let Err(e) = io::stdin().read_line(&mut user_input) {
            println!("Error reading input, please try again.  {}", e);
            println!("");
            continue;
        }
        
        let input_num_turns = user_input.trim();
        check_quit_input(input_num_turns);

        let num_turns: u32 =
            match input_num_turns.trim().parse() {
                Ok(num) if num < 1 => {
                    println!("Number of turns must be >= 1");
                    println!("");
                    continue;
                },
                Ok(num) if num > 500 => {
                    println!("Number of turns must be <= 500");
                    println!("");
                    continue;
                },
                Ok(num) => num,
                Err(msg) => {
                    println!("Invalid number of turns: {}", msg);
                    println!("");
                    continue;
                }
            };

        return (num_players, num_turns);
    };
}
