use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    if &k > a.iter().max().unwrap() {
        println!("IMPOSSIBLE");
        return;
    }

    if a.iter().any(|x| x == &k) {
        println!("POSSIBLE");
        return;
    }

    a.sort_unstable();
    let mut d = (1..n)
        .map(|i| a[i] - a[i - 1])
        .filter(|x| x != &0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    d.sort_unstable();

    if d.is_empty() {
        println!("IMPOSSIBLE");
        return;
    }

    let div = if (1..d.len()).all(|i| d[i] % d[0] == 0) {
        d[0]
    } else {
        1
    };

    if a.iter().filter(|x| x >= &&k).all(|y| (y - k) % div != 0) {
        println!("IMPOSSIBLE");
    } else {
        println!("POSSIBLE");
    }
}
