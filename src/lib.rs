mod utils;

use poker_calc::{card::Card, winrate};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn calc(
    c1: i32,
    s1: i32,
    c2: i32,
    s2: i32,
    c3: i32,
    s3: i32,
    c4: i32,
    s4: i32,
    c5: i32,
    s5: i32,
    c6: i32,
    s6: i32,
    c7: i32,
    s7: i32,
    players: i32,
    limit: i32,
) -> f64 {
    let mut rng = fastrand::Rng::new();
    let hand = [
        Card::new_from_num(c1 as u8, s1 as u8),
        Card::new_from_num(c2 as u8, s2 as u8),
    ];
    let mut board = vec![];
    if c3 != 0 {
        board.push(Card::new_from_num(c3 as u8, s3 as u8));
    }
    if c4 != 0 {
        board.push(Card::new_from_num(c4 as u8, s4 as u8));
    }
    if c5 != 0 {
        board.push(Card::new_from_num(c5 as u8, s5 as u8));
    }
    if c6 != 0 {
        board.push(Card::new_from_num(c6 as u8, s6 as u8));
    }
    if c7 != 0 {
        board.push(Card::new_from_num(c7 as u8, s7 as u8));
    }
    let result = winrate::calc(
        &mut rng,
        hand,
        &board,
        players as usize,
        limit as usize,
        false,
    );
    return result;
}
