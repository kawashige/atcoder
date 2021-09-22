use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(usize, usize, u32); m]
    }

    let mut dp = vec![0_usize; 1 << n];
    dp[0] = 1;

    let mut condition = vec![vec![]; n + 1];
    for (x, y, z) in xyz {
        condition[x].push((y, z));
    }

    for i in 0..(1 << n) {
        for j in 0..n {
            if i & 1 << j == 0
                && condition[((i | 1 << j) as usize).count_ones() as usize]
                    .iter()
                    .all(|&(y, z)| ((i | 1 << j) & ((1 << y) - 1) as usize).count_ones() <= z)
            {
                dp[i | 1 << j] += dp[i];
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
