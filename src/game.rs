#![doc(hidden)]

pub trait Game {
    fn preprocess(&mut self);
    fn cards_for_turn(&self, turn: u8) -> Vec<(&str, i32)>;
}
