use std::collections::HashMap;

fn main() {}

#[test]
fn example() {
    // Input: nums = [2,3,3,2,2,4,2,3,4]
    // Output: 4

    let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
    let test = Solution::min_operations(nums);
    assert_eq!(test, 4);
}
struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::<i32, u32>::new();

        nums.iter().for_each(|v| {
            let old_count = counter.get(v).unwrap_or(&0);
            counter.insert(v.clone(), *old_count + 1);
        });

        let mut ops_counter = 0;

        for (_, count) in counter.iter() {
            let ops = count / 3;
            let resto = count % 3;

            ops_counter += match resto {
                0 => ops,
                1 if ops > 0 => ops + 1,
                2 => ops + 1,
                _ => return -1,
            }
        }

        return ops_counter as i32;
    }
}
