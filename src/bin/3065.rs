impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().filter(|&num| *num < k).count() as i32
    }
}

struct Solution;

fn main() {
    let nums = vec![2, 11, 10, 1, 3];
    let k = 10;
    let result = Solution::min_operations(nums, k);
    println!("Number of operations: {}", result);
}
