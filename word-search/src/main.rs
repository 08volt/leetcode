use std::collections::{HashMap, HashSet, VecDeque};

fn main() {}

#[test]
fn example() {
    // Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'ABCCED'
    // Output: true
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();

    let test1 = Solution::exist(board, word);
    assert_eq!(test1, true);

    // Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'SEE'
    // Output: true
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "SEE".to_string();

    let test2 = Solution::exist(board, word);
    assert_eq!(test2, true);

    // Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'ABCB'
    // Output: false

    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCB".to_string();

    let test3 = Solution::exist(board, word);
    assert_eq!(test3, false);
}

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let h_board = Solution::convert_vec_to_hashmap(board);
        let v_word: Vec<char> = word.chars().collect();
        let first_word = Word::new();

        let mut word_list = VecDeque::<Word>::new();

        word_list.push_back(first_word);

        while let Some(w) = word_list.pop_back() {
            if w.already_used.len() == v_word.len() {
                return true;
            }

            let next_pos: Vec<(i32, i32)> = (match w.current_pos {
                Some((i, j)) => {
                    let v = vec![
                        (i + 1, j.clone()),
                        (i - 1, j.clone()),
                        (i.clone(), j + 1),
                        (i.clone(), j - 1),
                    ];
                    v
                }
                None => h_board.keys().cloned().collect(),
            })
            .iter()
            .filter(|pos| {
                h_board.contains_key(pos)
                    && !w.already_used.contains(pos)
                    && h_board
                        .get(pos)
                        .map(|c| c.clone() == v_word.get(w.already_used.len()).unwrap().clone())
                        .unwrap_or(false)
            })
            .cloned()
            .collect();

            next_pos.iter().for_each(|p| {
                word_list.push_front(w.add_pos(p));
            });

            // println!("\n --> {word_list:?}");
        }

        false
    }

    fn convert_vec_to_hashmap(board: Vec<Vec<char>>) -> HashMap<(i32, i32), char> {
        let mut res = HashMap::<(i32, i32), char>::new();

        for (i, r) in board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                res.insert((i as i32, j as i32), c.clone());
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
struct Word {
    current_pos: Option<(i32, i32)>,
    already_used: HashSet<(i32, i32)>,
}

impl Word {
    fn new() -> Self {
        Self {
            current_pos: None,
            already_used: HashSet::<(i32, i32)>::new(),
        }
    }

    fn add_pos(&self, pos: &(i32, i32)) -> Self {
        let mut new_already_used = self.already_used.clone();
        new_already_used.insert(pos.clone());
        Self {
            current_pos: Some(pos.clone()),
            already_used: new_already_used,
        }
    }
}
