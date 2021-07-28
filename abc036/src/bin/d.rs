use proconio::input;

fn recurse(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>, dp: &mut Vec<Vec<u64>>) {
    const M: u64 = 1_000_000_007;

    seen[i] = true;

    dp[i][0] = 1;
    dp[i][1] = 1;
    for next in &list[i] {
        if seen[*next] {
            continue;
        }
        recurse(*next, list, seen, dp);

        dp[i][0] *= (dp[*next][0] + dp[*next][1]) % M;
        dp[i][0] %= M;
        dp[i][1] *= dp[*next][0];
        dp[i][1] %= M;
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    const M: u64 = 1_000_000_007;

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut dp = vec![vec![0; 2]; n];

    recurse(0, &list, &mut vec![false; n], &mut dp);

    println!("{}", (dp[0][0] + dp[0][1]) % M);
}
