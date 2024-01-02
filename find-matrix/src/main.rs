use std::{collections::HashMap, convert::TryInto, ops::Deref, vec};

fn main() {
    let nums = vec![1, 3, 4, 1, 2, 3, 1];
    let res = Solution::find_matrix(nums.clone());
    println!("{:?}", nums);
    println!("{:?}", res);
}
struct Solution {}

impl Solution {
    fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![Vec::<i32>::new()];
        res.pop();
        let mut c = Counter::from(nums);

        while c.len() > 0 {
            res.push(c.remove_keys());
        }
        res
    }
}

struct Counter {
    counter: HashMap<i32, u32>,
}

impl From<Vec<i32>> for Counter {
    fn from(value: Vec<i32>) -> Self {
        let mut c = Counter {
            counter: HashMap::<i32, u32>::new(),
        };

        value.iter().for_each(|v| {
            c.add_number(v);
        });

        c
    }
}

impl Counter {
    fn add_number(&mut self, n: &i32) {
        let old_count = self.counter.get(n).unwrap_or(&0);
        self.counter.insert(n.clone(), *old_count + 1);
    }

    fn get_count(&self, n: &i32) -> u32 {
        self.counter.get(n).unwrap_or(&0).clone()
    }

    fn remove_one(&mut self, n: &i32) {
        let old_count = self.get_count(n);
        if old_count - 1 <= 0 {
            self.counter.remove(n);
        } else {
            self.counter.insert(n.clone(), old_count - 1);
        }
    }

    fn remove_keys(&mut self) -> Vec<i32> {
        let res: Vec<i32> = self.counter.keys().cloned().collect();
        for (k, _) in self.counter.clone().iter() {
            self.remove_one(&k)
        }

        res
    }

    fn len(&self) -> usize {
        self.counter.len()
    }
}
