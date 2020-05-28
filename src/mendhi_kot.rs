use itertools::Itertools;
use crate::game::*;
use crate::utils::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg;

pub struct MendhiKot<'a, T: rand::Rng> {
    playing_cards: Vec<(&'a str, i32)>,
    players: u8,
    cards_each: u8,
    rng: T,
}

impl MendhiKot<'_, rand_pcg::Pcg64> {
    pub fn new<'a>(seed: u64) -> MendhiKot<'a, rand_pcg::Pcg64> {
        MendhiKot {
            playing_cards: Vec::new(),
            players: 0,
            cards_each: 0,
            rng: rand_pcg::Pcg64::seed_from_u64(seed),
        }
    }

    fn create_playing_cards(&mut self, num_decks: u8) {
        let suits = ["H", "S", "C", "D"];
        let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut deck: Vec<(&str, i32)> = suits.iter().cartesian_product(nums.iter()).map(|(&s, &n)| (s, n)).collect();
        let total_cards = 52;
        let mut cards_each: u8 = total_cards / self.players;
        let mut discard: u8 = 0;
        for i in (1..=cards_each).rev() {
            discard = total_cards - self.players * i;
            if discard % 4 == 0 {
                cards_each = i;
                break;
            }
        }
        for suit in &suits {
            for i in 0..(discard / 4) {
                deck.remove(deck.iter().position(|(s, n)| (s, n) == (suit, &(i as i32))).unwrap());
            }
        }
        for _ in 0..(num_decks - 1) {
            deck.extend(deck.clone());
        }
        self.playing_cards = deck;
        self.cards_each = cards_each * 2;
    }

    fn shuffle(&mut self) {
        self.playing_cards.shuffle(&mut self.rng);
    }
}

impl Game for MendhiKot<'_, rand_pcg::Pcg64> {
    fn preprocess(&mut self) {
        self.players = loop {
            let players = get_players();
            if players % 2 == 0 {
                break players
            }
            else {
                println!("Must have even number of players for Mendhi Kot");
                continue
            }
        };
        self.create_playing_cards(get_num_decks());
        self.shuffle();
    }

    fn cards_for_turn(&self, turn: u8) -> Vec<(&str, i32)> {
        if turn > self.players {
            println!("Invalid turn, must be less than number of players ({})", self.players);
            return Vec::<(&str, i32)>::new();
        }
        let idx1: usize = ((turn - 1) * self.cards_each) as usize;
        let idx2: usize = idx1 + self.cards_each as usize;
        let mut my_cards = self.playing_cards[idx1..idx2].to_vec();
        my_cards.sort();
        my_cards.to_vec()
    }
}
