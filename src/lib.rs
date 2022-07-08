mod utils;

use poker_calc::{card::Card, winrate};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poker-calc!");
}

#[wasm_bindgen]
pub fn calc(hand_str: String, board_str: String) -> f64 {
    let (left, right) = hand_str.split_at(2);
    let hand = [Card::new(left), Card::new(right)];
    let mut board = Vec::new();
    if board_str.chars().count() == 10 {
        board.push(Card::new(&board_str[0..2]));
        board.push(Card::new(&board_str[2..4]));
        board.push(Card::new(&board_str[4..8]));
        board.push(Card::new(&board_str[8..10]));
    }
    let mut rng = fastrand::Rng::new();
    let result = winrate::calc(&mut rng, hand, &board, 6, 1000000, false);
    println!("{}", result);
    return result;
}
