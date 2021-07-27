use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    const M: usize = 1_000_000_007;

    let mut nums = Vec::new();
    for i in 0..h {
        for j in 0..w {
            nums.push((a[i][j], (i, j)));
        }
    }
    nums.sort_unstable();

    let mut dp = vec![vec![1; w]; h];

    for (_, (i, j)) in nums {
        for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (r, c) = (i as i32 + x, j as i32 + y);
            if r < 0
                || r >= h as i32
                || c < 0
                || c >= w as i32
                || a[r as usize][c as usize] <= a[i][j]
            {
                continue;
            }
            dp[r as usize][c as usize] += dp[i][j];
            dp[r as usize][c as usize] %= M;
        }
    }

    println!(
        "{}",
        dp.into_iter().flatten().fold(0, |acc, x| (x + acc) % M)
    );
}
