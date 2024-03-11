impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        for j in 0..matrix[0].len() {
            let mut mx = 0;
            for row in matrix.iter() {
                mx = mx.max(row[j]);
            }
            for row in matrix.iter_mut() {
                if row[j] == -1 {
                    row[j] = mx;
                }
            }
        }
        matrix
    }
}

struct Solution;
fn main() {
    let matrix = vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]];
    let modified = Solution::modified_matrix(matrix);
    println!("{:?}", modified);
}
