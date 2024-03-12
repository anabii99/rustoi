use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup = HashMap::new();

        for (i, x) in nums.iter().enumerate() {
            if let Some(&j) = lookup.get(&(target - x)) {
                return vec![j as i32, i as i32];
            }

            lookup.insert(x, i);
        }

        vec![]
    }
}

struct Solution;

fn main() {
    let nums = vec![2, 7, 11, 15, 30, 21];
    let target = 51;
    println!("{:?}", Solution::two_sum(nums, target));
}
