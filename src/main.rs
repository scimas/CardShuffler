use std::io;
use std::collections::HashMap;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_pcg;
use itertools::Itertools;

fn main() {
    let seed: u64 = loop {
        let mut seed = String::new();
        println!("Enter seed:");
        io::stdin().read_line(&mut seed).expect("Couldn't read the seed");
        let seed: i64 = match seed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad characters in seed, make sure it is a positive integer");
                continue
            },
        };
        if seed <= 0 {
            println!("Seed must be a positive integer");
        }
        else {
            break seed as u64;
        }
    };
    let (num_cards, turn): (u8, u8) = loop {
        let mut num_cards = String::new();
        let mut turn = String::new();
        println!("Enter how many cards for each player:");
        io::stdin().read_line(&mut num_cards).expect("Couldn't read the number of cards");
        let num_cards: i8 = match num_cards.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad characters in number of cards, make sure it is a positive integer");
                continue
            },
        };
        if num_cards <= 0 {
            println!("Bad characters in number of cards, make sure it is a positive integer");
            continue
        }
        println!("Enter your turn:");
        io::stdin().read_line(&mut turn).expect("Couldn't read the turn");
        let turn: i8 = match turn.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad characters in turn, make sure it is a positive integer");
                continue
            }
        };
        if turn <= 0 {
            println!("Turn must be a positive integer");
            continue
        }
        if turn * num_cards > 52 {
            println!("You are asking for more than 52 cards here");
            println!("Make sure both number of cards and turn are correct");
            continue
        }
        else {
            break (num_cards as u8, turn as u8);
        }
    };
    
    let suits = ["H", "S", "C", "D"];
    let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
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
    
    let mut rng = rand_pcg::Pcg64::seed_from_u64(seed);
    let mut deck: Vec<(&str, i32)> = suits.iter().cartesian_product(nums.iter()).map(|(&s, &n)| (s, n)).collect();
    deck.shuffle(&mut rng);
    
    let idx1: usize = (num_cards * (turn - 1)) as usize;
    let idx2: usize = (num_cards * turn) as usize;
    let my_cards = &mut deck[idx1..idx2];
    my_cards.sort();
    println!("Suit    Card");
    for &mut card in my_cards {
        println!("{}       {}", card.0, num_map.get(&card.1).unwrap())
    }
}
