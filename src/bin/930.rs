impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut record = std::collections::HashMap::new();
        let mut sum = 0;
        let mut ret = 0;
        let _ = *record.entry(0).or_insert(1);
        for &num in &nums {
            sum += num;
            if let Some(count) = record.get(&(sum - goal)) {
                ret += count;
            }
            *record.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}

struct Solution;

fn main() {
    let nums = vec![1, 0, 1, 0, 1];
    let goal = 2;
    let result = Solution::num_subarrays_with_sum(nums, goal);

    println!("{:#?}", result);
}
