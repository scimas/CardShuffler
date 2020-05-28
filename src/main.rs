use std::io;
use std::collections::HashMap;

mod utils;
mod game;
mod badam_sat;
mod judgement;
mod pach_tin_don;

// Get seed value from user and return it as u64. Keep trying until valid input
// is obtained.
fn get_seed() -> u64 {
    loop {
        let mut seed = String::new();
        println!("Enter seed:");
        io::stdin().read_line(&mut seed).expect("Couldn't read the seed");
        let seed: i64 = match seed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                utils::bad_char_err("seed");
                continue
            },
        };
        if seed <= 0 {
            utils::must_be_positive("Seed");
        }
        else {
            break seed as u64
        }
    }
}

// Get which game is being played from user. Input is an integer that is mapped
// to the `Game` enum. Keep trying until valid input is obtained.
fn get_game(seed: u64) -> Box<dyn game::Game> {
    loop {
        let mut game = String::new();
        println!("Which game do you want to play? Enter corresponding number:");
        println!("1: Judgement");
        println!("2: Badam Sat");
        println!("3: Pach Tin Don");
        io::stdin().read_line(&mut game).expect("Couldn't read the game number");
        match game.trim().parse::<i32>() {
            Ok(num) => {
                match num {
                    1 => break Box::new(judgement::Judgement::new(seed)),
                    2 => break Box::new(badam_sat::BadamSat::new(utils::get_players(), seed)),
                    3 => break Box::new(pach_tin_don::PachTinDon::new(seed)),
                    _ => {
                        println!("Invalid game code, try again");
                        continue
                    },
                }
            }
            Err(_) => {
                utils::bad_char_err("game number");
                continue
            }
        }
    }
}

// Get which turn the user is playing on and return it as u8. Keep trying until
// valid input is obtained.
fn get_turn() -> u8 {
    loop {
        let mut turn = String::new();
        println!("Enter your turn, (q to quit):");
        io::stdin().read_line(&mut turn).expect("Couldn't read the turn");
        let turn: i8 = match turn.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if turn == "q\n" {
                    break 0
                }
                utils::bad_char_err("turn");
                continue
            }
        };
        if turn <= 0 {
            utils::must_be_positive("Turn");
            continue
        }
        else {
            break turn as u8
        }
    }
}

fn main() {
    let seed = get_seed();
    let mut game = get_game(seed);
    
    // A hash map for displaying numerical values of cards as proper symbols,
    // like Jack, Queen and King.
    let mut num_map: HashMap<i32, String> = HashMap::new();
    num_map.entry(0).or_insert(String::from("2"));
    num_map.entry(1).or_insert(String::from("3"));
    num_map.entry(2).or_insert(String::from("4"));
    num_map.entry(3).or_insert(String::from("5"));
    num_map.entry(4).or_insert(String::from("6"));
    num_map.entry(5).or_insert(String::from("7"));
    num_map.entry(6).or_insert(String::from("8"));
    num_map.entry(7).or_insert(String::from("9"));
    num_map.entry(8).or_insert(String::from("10"));
    num_map.entry(9).or_insert(String::from("J"));
    num_map.entry(10).or_insert(String::from("Q"));
    num_map.entry(11).or_insert(String::from("K"));
    num_map.entry(12).or_insert(String::from("A"));
    
    game.preprocess();
    loop {
        let turn = get_turn();
        if turn == 0 {
            break;
        }
        println!("Suit    Card");
        for card in game.cards_for_turn(turn) {
            println!("{}       {}", card.0, num_map.get(&card.1).unwrap());
        }
    }
}
