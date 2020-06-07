use itertools::Itertools;
use crate::game::*;
use crate::utils::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg;

pub struct Judgement<'a, T: rand::Rng> {
    playing_cards: Vec<(&'a str, i32)>,
    players: u8,
    cards_each: u8,
    rng: T,
}

impl Judgement<'_, rand_pcg::Pcg64> {
    pub fn new<'a>(seed: u64) -> Judgement<'a, rand_pcg::Pcg64> {
        Judgement {
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
        for _ in 0..(num_decks - 1) {
            deck.extend(deck.clone());
        }
        self.playing_cards = deck;
    }

    fn get_cards_each(&mut self) {
        self.cards_each = get_num_cards();
    }

    fn shuffle(&mut self) {
        self.playing_cards.shuffle(&mut self.rng);
    }
}

impl Game for Judgement<'_, rand_pcg::Pcg64> {
    fn preprocess(&mut self) {
        self.players = get_players();
        self.create_playing_cards(get_num_decks());
        self.get_cards_each();
        self.shuffle();
    }

    fn cards_for_turn(&self, turn: u8) -> Vec<(&str, i32)> {
        if turn > self.players {
            println!("Invalid turn, must not be more number of players ({})", self.players);
            return Vec::<(&str, i32)>::new();
        }
        let idx1: usize = ((turn - 1) * self.cards_each) as usize;
        let idx2: usize = idx1 + self.cards_each as usize;
        let mut my_cards = self.playing_cards[idx1..idx2].to_vec();
        my_cards.sort();
        my_cards.to_vec()
    }
}
