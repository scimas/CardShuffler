//! # Card Shuffler
//!
//! `card_shuffler` allows you to shuffle cards in a deterministically. If you
//! want to play with friends remotely - but don't have a way to deal cards -
//! this will help you.
//!
//! Currently supports बदाम सात (Badam Sat), Judgement, मेंढी कोट (Mendhi Kot)
//! and पाच - तीन - दोन (5 - 3 - 2).
//!
//! Run `card_shuffler[.exe]` and follow on screen instructions.
//!
//! `Seed` is a number used to shuffle the cards. The cards will be shuffled
//! exactly in the same order whenever you use the same number for the seed.

use std::collections::HashMap;
use std::io;

mod badam_sat;
mod game;
mod judgement;
mod mendhi_kot;
mod pach_tin_don;
mod utils;

// Get seed value from user and return it as u64. Keep trying until valid input
// is obtained.
#[doc(hidden)]
fn get_seed() -> u64 {
    loop {
        let mut seed = String::new();
        println!("Enter seed:");
        io::stdin()
            .read_line(&mut seed)
            .expect("Couldn't read the seed");
        let seed: i64 = match seed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                utils::bad_char_err("seed");
                continue;
            }
        };
        if seed <= 0 {
            utils::must_be_positive("Seed");
        } else {
            break seed as u64;
        }
    }
}

// Get which game is being played from user. Input is an integer that is mapped
// to the `Game` enum. Keep trying until valid input is obtained.
#[doc(hidden)]
fn get_game(seed: u64) -> Box<dyn game::Game> {
    let games = ["Judgement", "Badam Sat", "Pach Tin Don", "Mendhi Kot"];
    loop {
        let mut game = String::new();
        println!("Which game do you want to play? Enter corresponding number");
        for (i, g) in games.iter().enumerate() {
            println!("{}: {}", i + 1, g);
        }
        io::stdin()
            .read_line(&mut game)
            .expect("Couldn't read the game number");
        match game.trim().parse::<i32>() {
            Ok(num) => match num {
                1 => break Box::new(judgement::Judgement::new(seed)),
                2 => break Box::new(badam_sat::BadamSat::new(utils::get_players(), seed)),
                3 => break Box::new(pach_tin_don::PachTinDon::new(seed)),
                4 => break Box::new(mendhi_kot::MendhiKot::new(seed)),
                _ => {
                    println!("Invalid game code, try again");
                    continue;
                }
            },
            Err(_) => {
                utils::bad_char_err("game number");
                continue;
            }
        }
    }
}

// Get which turn the user is playing on and return it as u8. Keep trying until
// valid input is obtained.
#[doc(hidden)]
fn get_turn() -> u8 {
    loop {
        let mut turn = String::new();
        println!("Enter your turn, (q to quit):");
        io::stdin()
            .read_line(&mut turn)
            .expect("Couldn't read the turn");
        let turn: i8 = match turn.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if &turn[0..1] == "q" {
                    break 0;
                }
                utils::bad_char_err("turn");
                continue;
            }
        };
        if turn <= 0 {
            utils::must_be_positive("Turn");
            continue;
        } else {
            break turn as u8;
        }
    }
}

#[doc(hidden)]
fn main() {
    let seed = get_seed();
    let mut game = get_game(seed);
    // A hash map for displaying numerical values of cards as proper symbols,
    // like Jack, Queen and King.
    let mut num_map: HashMap<i32, String> = HashMap::new();
    num_map.insert(0, "2".to_owned());
    num_map.insert(1, "3".to_owned());
    num_map.insert(2, "4".to_owned());
    num_map.insert(3, "5".to_owned());
    num_map.insert(4, "6".to_owned());
    num_map.insert(5, "7".to_owned());
    num_map.insert(6, "8".to_owned());
    num_map.insert(7, "9".to_owned());
    num_map.insert(8, "10".to_owned());
    num_map.insert(9, "J".to_owned());
    num_map.insert(10, "Q".to_owned());
    num_map.insert(11, "K".to_owned());
    num_map.insert(12, "A".to_owned());
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
