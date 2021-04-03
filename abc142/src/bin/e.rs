use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut keys = HashMap::new();
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: [usize; b]
        }
        let mut key = 0;
        for i in c {
            key |= 1 << (i - 1);
        }

        let entry = keys.entry(key).or_insert(a);
        *entry = std::cmp::min(a, *entry);
    }

    let keys = keys.into_iter().collect::<Vec<(usize, usize)>>();
    let m = keys.len();

    let mut dp = vec![1_000_000_000; 5000];
    dp[0] = 0;

    for i in 0..m {
        for j in 0..(1 << n) {
            dp[j | keys[i].0] = std::cmp::min(dp[j | keys[i].0], dp[j] + keys[i].1);
        }
    }

    if dp[(1 << n) - 1] >= 1_000_000_000 {
        println!("-1");
    } else {
        println!("{}", dp[(1 << n) - 1]);
    }
}
