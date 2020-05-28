use itertools::Itertools;
use crate::game::*;
use crate::utils::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg;

pub struct BadamSat<'a, T: rand::Rng> {
    playing_cards: Vec<(&'a str, i32)>,
    players: u8,
    cards_dist: Vec<u8>,
    rng: T,
}

impl BadamSat<'_, rand_pcg::Pcg64> {
    pub fn new<'a>(players: u8, seed: u64) -> BadamSat<'a, rand_pcg::Pcg64> {
        BadamSat {
            playing_cards: Vec::new(),
            players,
            cards_dist: Vec::new(),
            rng: rand_pcg::Pcg64::seed_from_u64(seed),
        }
    }

    fn create_playing_cards(&mut self, num_decks: u8) {
        let suits = ["H", "S", "C", "D"];
        let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut deck: Vec<(&str, i32)> = suits.iter().cartesian_product(nums.iter()).map(|(&s, &n)| (s, n)).collect();
        for _ in 0..(num_decks - 1) {
            deck.extend(deck.clone());
        }
        self.playing_cards = deck;
    }

    fn create_cards_dist(&mut self) {
        let qtn: u8 = self.playing_cards.len().div_euclid(self.players as usize) as u8;
        let rem: u8 = self.playing_cards.len().rem_euclid(self.players as usize) as u8;
        if rem == 0 {
            self.cards_dist = vec![qtn; self.players as usize]
        }
        else {
            let mut num_cards = vec![qtn; self.players as usize];
            for i in 0..rem as usize {
                num_cards[i] += 1;
            }
            self.cards_dist = num_cards;
        }
    }

    fn shuffle(&mut self) {
        self.playing_cards.shuffle(&mut self.rng);
    }
}

impl Game for BadamSat<'_, rand_pcg::Pcg64> {
    fn preprocess(&mut self) {
        self.create_playing_cards(get_num_decks());
        self.create_cards_dist();
        self.shuffle();
    }

    fn cards_for_turn(&self, turn: u8) -> Vec<(&str, i32)> {
        if turn > self.players {
            println!("Invalid turn, must be less than number of players ({})", self.players);
            return Vec::<(&str, i32)>::new();
        }
        let mut idx1: usize = 0;
        for i in 0..(turn - 1) as usize {
            idx1 += self.cards_dist[i] as usize;
        }
        let idx2: usize = idx1 + self.cards_dist[(turn - 1) as usize] as usize;
        let mut my_cards = self.playing_cards[idx1..idx2].to_vec();
        my_cards.sort();
        my_cards.to_vec()
    }
}
