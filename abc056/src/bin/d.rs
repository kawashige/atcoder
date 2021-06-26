use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut set = HashSet::new();
    let mut dp = vec![HashSet::new(); k];

    let mut count = n;

    for i in 0..n {
        if a[i] >= k {
            count -= 1;
            continue;
        }

        for j in (0..dp.len()).rev() {
            if j > 0 && dp[j].is_empty() {
                continue;
            }
            if j + a[i] >= k {
                dp[j].iter().for_each(|k| {
                    set.insert(*k);
                });
                set.insert(i);
            } else {
                for x in dp[j].clone() {
                    dp[j + a[i]].insert(x);
                }
                dp[j + a[i]].insert(i);
            }
        }
    }

    println!("{}", count - set.len());
}
