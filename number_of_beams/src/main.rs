use std::collections::HashMap;

fn main() {}

struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut beam_counter: i32 = 0;

        bank.iter().fold(0, |c, r| {
            let current_count = r.chars().filter(|c| c == &'1').count() as i32;
            if current_count > 0 {
                beam_counter += c * current_count;
                current_count
            } else {
                c
            }
        });

        beam_counter
    }
}
