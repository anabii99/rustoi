impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut left_sum = 0;
        let mut right_sum = (n * (n + 1)) / 2;
        for i in 1..=n {
            right_sum -= i;
            if left_sum == right_sum {
                return i as i32;
            }
            left_sum += i;
        }
        -1
    }
}

struct Solution;

fn main() {
    let n = 8;
    println!(
        "The pivot integer for {} is: {}",
        n,
        Solution::pivot_integer(n)
    );
}
