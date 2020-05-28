use std::io;

// Utility to print error about having bad characters in user input.
pub fn bad_char_err(param: &str) {
    println!("Bad characters in {}, make sure it is a positive integer", param);
}

// Utility to print error about some input parameter not being positive.
// Most parameters are expected to be positive in this crate.
pub fn must_be_positive(param: &str) {
    println!("{} must be positive", param);
}

// Get number of players playing the game from user and return it as u8. Keep
// trying until valid input is obtained.
pub fn get_players() -> u8 {
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
            continue
        }
        else {
            break num_players as u8
        }
    }
}

// Get number of decks from user and return it as u8. Keep trying until valid
// input is obtained.
pub fn get_num_decks() -> u8 {
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

// Get number of cards from user (in games where everyone gets equal number of
// cards) and return it as u8. Keep trying until valid input is obtained.
pub fn get_num_cards() -> u8 {
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

// Get dealing round from user and return it as u8. Keep trying until valid
// input is obtained.
pub fn get_dealing_round() -> u8 {
    loop {
        let mut dealing_round = String::new();
        println!("Enter number of the dealing round:");
        io::stdin().read_line(&mut dealing_round).expect("Couldn't read the dealing round");
        let dealing_round: i8 = match dealing_round.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                bad_char_err("dealing round");
                continue
            },
        };
        if dealing_round <= 0 || dealing_round > 2 {
            println!("Dealing round must be 1 or 2");
            continue
        }
        else {
            break dealing_round as u8
        }
    }
}
