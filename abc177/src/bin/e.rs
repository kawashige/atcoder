use proconio::input;
use std::collections::HashSet;

fn prime_factors(mut n: usize, spf: &Vec<usize>) -> HashSet<usize> {
    let mut factors = HashSet::new();
    while n != 1 {
        factors.insert(spf[n]);
        n /= spf[n]
    }
    factors
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let max = 1_000_000;

    let mut spf = (0..=max).collect::<Vec<usize>>();
    for i in 2..=max {
        if spf[i] == i {
            for j in ((i * 2)..=max).step_by(i) {
                spf[j] = i;
            }
        }
    }

    let mut prime_factor_count = vec![0; max + 1];
    let mut max_f = 0;
    for i in 0..a.len() {
        for f in prime_factors(a[i], &spf) {
            max_f = std::cmp::max(max_f, f);
            prime_factor_count[f] += 1;
        }
    }

    if prime_factor_count[..=max_f]
        .iter()
        .all(|i| i == &0 || i == &1)
    {
        println!("pairwise coprime");
    } else if prime_factor_count[..=max_f].iter().all(|i| *i != a.len()) {
        println!("setwise coprime");
    } else {
        println!("not coprime")
    }
}
