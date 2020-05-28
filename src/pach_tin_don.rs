use itertools::Itertools;
use crate::game::*;
use crate::utils::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg;

pub struct PachTinDon<'a, T: rand::Rng> {
    playing_cards: Vec<(&'a str, i32)>,
    cards_each: u8,
    rng: T,
}

impl PachTinDon<'_, rand_pcg::Pcg64> {
    pub fn new<'a>(seed: u64) -> PachTinDon<'a, rand_pcg::Pcg64> {
        PachTinDon {
            playing_cards: Vec::new(),
            cards_each: 10,
            rng: rand_pcg::Pcg64::seed_from_u64(seed),
        }
    }
    
    fn create_playing_cards(&mut self) {
        let suits = ["H", "S", "C", "D"];
        let nums = [6, 7, 8, 9, 10, 11, 12];
        let mut deck: Vec<(&str, i32)> = suits.iter().cartesian_product(nums.iter()).map(|(&s, &n)| (s, n)).collect();
        deck.push(("H", 5));
        deck.push(("S", 5));
        self.playing_cards = deck;
    }

    fn shuffle(&mut self) {
        self.playing_cards.shuffle(&mut self.rng);
    }
}

impl Game for PachTinDon<'_, rand_pcg::Pcg64> {
    fn preprocess(&mut self) {
        self.create_playing_cards();
        self.shuffle();
    }

    fn cards_for_turn(&self, turn: u8) -> Vec<(&str, i32)> {
        if turn > 3 {
            println!("Invalid turn, must be {{1, 2, 3}}");
            return Vec::<(&str, i32)>::new();
        }
        let dealing_round = get_dealing_round();
        let idx1: usize = (15 * (dealing_round - 1) + (turn - 1) * self.cards_each / 2) as usize;
        let idx2: usize = idx1 + (self.cards_each / 2) as usize;
        let mut my_cards = self.playing_cards[idx1..idx2].to_vec();
        my_cards.sort();
        my_cards.to_vec()
    }
}
