use std::io;
use std::collections::HashMap;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_pcg;
use itertools::Itertools;

// Types of games supported by the crate.
enum Game {
    Judgement,
    BadamSat,
}

// Utility to print error about having bad characters in user input.
fn bad_char_err(param: &str) {
    println!("Bad characters in {}, make sure it is a positive integer", param);
}

// Utility to print error about some input parameter not being positive.
// Most parameters are expected to be positive in this crate.
fn must_be_positive(param: &str) {
    println!("{} must be positive", param);
}

// Combination of cards for each player and number of player exceeds available cards in given decks. Print and error about it.
fn too_many_cards() {
    println!("You're asking for more cards than available in these decks, try again");
}

fn get_seed() -> u64 {
    loop {
        let mut seed = String::new();
        println!("Enter seed:");
        io::stdin().read_line(&mut seed).expect("Couldn't read the seed");
        let seed: i64 = match seed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("seed");
                continue
            },
        };
        if seed <= 0 {
            must_be_positive("Seed");
        }
        else {
            break seed as u64
        }
    }
}

fn get_num_decks() -> u8 {
    loop {
        let mut num_decks = String::new();
        println!("Enter number of decks:");
        io::stdin().read_line(&mut num_decks).expect("Couldn't read the number of decks");
        let num_decks: i64 = match num_decks.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("number of decks");
                continue
            },
        };
        if num_decks <= 0 {
            must_be_positive("Number of decks");
        }
        else {
            break num_decks as u8
        }
    }
}

fn get_num_cards() -> u8 {
    loop {
        let mut num_cards = String::new();
        println!("Enter number of cards for each player:");
        io::stdin().read_line(&mut num_cards).expect("Couldn't read the number of cards");
        let num_cards: i8 = match num_cards.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("number of cards");
                continue
            },
        };
        if num_cards <= 0 {
            must_be_positive("Number of cards");
        }
        else {
            break num_cards as u8
        }
    }
}

fn get_game() -> Game {
    loop {
        let mut game = String::new();
        println!("Which game do you want to play? Enter corresponding number:");
        println!("1: Judgement");
        println!("2: Badam Sat");
        io::stdin().read_line(&mut game).expect("Couldn't read the game number");
        match game.trim().parse::<i32>() {
            Ok(num) => {
                match num {
                    1 => break Game::Judgement,
                    2 => break Game::BadamSat,
                    _ => continue,
                }
            }
            Err(_) => {
                bad_char_err("game number");
                continue
            }
        }
    }
}

fn get_players() -> u8 {
    loop {
        let mut num_players = String::new();
        println!("Enter number of players in the game:");
        io::stdin().read_line(&mut num_players).expect("Couldn't read the number of players");
        let num_players: i8 = match num_players.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("num_players");
                continue
            },
        };
        if num_players <= 0 {
            must_be_positive("Number of players");
        }
        else {
            break num_players as u8
        }
    }
}

fn num_cards_for(game: &Game, num_decks: u8, num_players: u8) -> Vec<u8> {
    match game {
        Game::Judgement => loop {
            let num_cards = get_num_cards();
            let deck_size = 52;
            if num_cards * num_players > num_decks * deck_size {
                too_many_cards();
                continue
            }
            else {
                break vec![num_cards; num_players as usize]
            }
        },
        Game::BadamSat => {
            let deck_size = 52;
            let qtn = (num_decks * deck_size).div_euclid(num_players);
            let rem = (num_decks * deck_size).rem_euclid(num_players);
            if rem == 0 {
                vec![qtn; num_players as usize]
            }
            else {
                let mut num_cards = vec![qtn; num_players as usize];
                for i in 0..rem as usize {
                    num_cards[i] += 1;
                }
                num_cards
            }
        },
    }
}

fn get_turn(num_players: u8) -> u8 {
    loop {
        let mut turn = String::new();
        println!("Enter your turn:");
        io::stdin().read_line(&mut turn).expect("Couldn't read the turn");
        let turn: i8 = match turn.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("turn");
                continue
            }
        };
        if turn <= 0 {
            must_be_positive("Turn");
            continue
        }
        else if turn > num_players as i8 {
            println!("Turn number can't be more than number of players");
            continue
        }
        else {
            break turn as u8
        }
    }
}

fn get_deck(game: &Game, num_decks: u8) -> Vec<(&'static str, i32)> {
    let suits = ["H", "S", "C", "D"];
    let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut deck: Vec<(&str, i32)> = suits.iter().cartesian_product(nums.iter()).map(|(&s, &n)| (s, n)).collect();
    match game {
        Game::Judgement => {
            for _ in 0..(num_decks - 1) {
                deck.extend(deck.clone());
            }
            deck
        },
        Game::BadamSat => {
            for _ in 0..(num_decks - 1) {
                deck.extend(deck.clone());
            }
            deck
        }
    }
}

fn main() {
    let seed = get_seed();
    let game = get_game();
    let num_decks = get_num_decks();
    let num_players = get_players();
    let turn = get_turn(num_players);
    let num_cards = num_cards_for(&game, num_decks, num_players);
    
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
    let mut deck = get_deck(&game, num_decks);
    let mut rng = rand_pcg::Pcg64::seed_from_u64(seed);
    deck.shuffle(&mut rng);
    
    let mut idx1: usize = 0;
    for i in 0..(turn - 1) as usize {
        idx1 += num_cards[i] as usize;
    }
    let idx2: usize = idx1 + num_cards[(turn - 1) as usize] as usize;
    let my_cards = &mut deck[idx1..idx2];
    my_cards.sort();
    println!("Suit    Card");
    for &mut card in my_cards {
        println!("{}       {}", card.0, num_map.get(&card.1).unwrap())
    }
}
