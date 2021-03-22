use proconio::input;
use std::collections::HashMap;

fn modinv(a: usize) -> usize {
    let mut a = a as i64;
    let m = 1_000_000_007;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as usize
}

fn prime_factors(mut n: usize, spf: &Vec<usize>, count: &mut Vec<usize>) {
    let mut map = HashMap::new();
    while n != 1 {
        *map.entry(spf[n]).or_insert(0) += 1;
        n /= spf[n];
    }
    for (num, c) in map {
        count[num] = std::cmp::max(count[num], c);
    }
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

    let mut count = vec![0; max + 1];
    for i in 0..n {
        prime_factors(a[i], &spf, &mut count);
    }

    let mut lcm = 1;
    for i in 1..count.len() {
        if count[i] == 0 {
            continue;
        }
        lcm = lcm * i.pow(count[i] as u32) % 1_000_000_007;
    }

    let mut sum = 0;
    for i in 0..n {
        sum = (sum + lcm * modinv(a[i])) % 1_000_000_007;
    }

    println!("{}", sum);
}
