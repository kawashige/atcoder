use proconio::input;

fn recurse(i: usize, list: &Vec<Vec<usize>>, c: &Vec<char>, dp: &mut Vec<Vec<usize>>) {
    const M: usize = 1_000_000_007;

    dp[i][2] = 1;
    if c[i] == 'a' {
        dp[i][0] = 1;
        dp[i][1] = 0;
    } else {
        dp[i][0] = 0;
        dp[i][1] = 1;
    }

    for child in &list[i] {
        if dp[*child][0] != std::usize::MAX {
            continue;
        }

        recurse(*child, list, c, dp);

        if c[i] == 'a' {
            dp[i][0] *= (dp[*child][0] + dp[*child][2]) % M;
            dp[i][0] %= M;
        } else {
            dp[i][1] *= (dp[*child][1] + dp[*child][2]) % M;
            dp[i][1] %= M;
        }
        dp[i][2] *= (dp[*child][0] + dp[*child][1] + 2 * dp[*child][2]) % M;
        dp[i][2] %= M;
    }

    dp[i][2] = (M + dp[i][2] - if c[i] == 'a' { dp[i][0] } else { dp[i][1] }) % M;
}

fn main() {
    input! {
        n: usize,
        c: [char; n],
        ab: [(usize, usize); n - 1]
    }

    let mut list = vec![vec![]; n];
    for (a, b) in ab {
        list[a - 1].push(b - 1);
        list[b - 1].push(a - 1);
    }

    let mut dp = vec![vec![std::usize::MAX; 3]; n];
    recurse(0, &list, &c, &mut dp);

    println!("{}", dp[0][2]);
}
