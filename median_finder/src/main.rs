use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {}

#[test]
fn example() {
    //["MedianFinder","addNum","addNum","findMedian","addNum","findMedian"]
    // [[],[1],[2],[],[3],[]]

    // [null,null,null,1.50000,null,2.00000]

    let mut m = MedianFinder::new();
    m.add_num(1);
    println!("{m:?}");
    m.add_num(2);
    println!("{m:?}");
    assert_eq!(m.find_median(), 1.5);
    m.add_num(3);
    assert_eq!(m.find_median(), 2.0);

    // let test3 = Solution::exist(board, word);
    // assert_eq!(test3, false);
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[derive(Debug)]
struct MedianFinder {
    left: BinaryHeap<i32>,           // max heap
    right: BinaryHeap<Reverse<i32>>, // min heap
    median: Option<f64>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left: BinaryHeap::<i32>::new(),
            right: BinaryHeap::<Reverse<i32>>::new(),
            median: None,
        }
    }

    fn add_num(&mut self, num: i32) {
        match self.median {
            Some(m) if m > num as f64 => self.left.push(num),
            _ => self.right.push(Reverse(num)),
        };

        if self.left.len() > self.right.len() {
            self.right.push(Reverse(self.left.pop().unwrap()));
        }
        if self.right.len() - 1 > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }

        if self.right.len() == self.left.len() {
            let r = self.right.peek().unwrap().0;
            let l = self.left.peek().unwrap();
            self.median = Some((r as f64 + l.clone() as f64) / 2.0);
        } else {
            self.median = Some(self.right.peek().unwrap().0 as f64);
        }
    }

    fn find_median(&self) -> f64 {
        self.median.unwrap_or_default()
    }
}
