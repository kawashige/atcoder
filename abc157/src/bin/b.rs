use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        board: [[usize; 3]; 3],
        n: usize,
        nums: [usize; n]
    }

    let nums = nums.into_iter().collect::<HashSet<usize>>();

    if (0..3).any(|i| (0..3).all(|j| nums.contains(&board[i][j])))
        || (0..3).any(|i| (0..3).all(|j| nums.contains(&board[j][i])))
        || (nums.contains(&board[0][0])
            && nums.contains(&board[1][1])
            && nums.contains(&board[2][2]))
        || (nums.contains(&board[0][2])
            && nums.contains(&board[1][1])
            && nums.contains(&board[2][0]))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
