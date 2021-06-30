use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut p: u64,
        a: [u64; n]
    }

    let mut r = 0_u64;

    let h = n / 2;
    let mut left = HashMap::new();

    for i in 1..2_usize.pow(h as u32) {
        let c = i.count_ones() as usize;
        if c > k {
            continue;
        }
        let mut sum = 0;
        for j in 0..h {
            if i & 1 << j > 0 {
                sum += a[j];
            }
        }

        if c == k {
            if sum <= p {
                r += 1;
            }
        } else if c < k {
            (*left.entry(i.count_ones() as usize).or_insert(Vec::new())).push(sum);
        }
    }

    for i in 0..k {
        if let Some(v) = left.get_mut(&i) {
            v.sort_unstable();
        }
    }

    for i in 1..2_usize.pow((n - h) as u32) {
        let c = i.count_ones() as usize;
        if c > k {
            continue;
        }
        let mut sum = 0;
        for j in 0..=(n - h) {
            if i & 1 << j > 0 {
                sum += a[h + j];
            }
        }

        if c == k {
            if sum <= p {
                r += 1;
            }
        } else if c < k && sum < p {
            if let Some(v) = left.get(&(k - c)) {
                r += match v.binary_search(&(p - sum)) {
                    Ok(x) => x + 1,
                    Err(x) => x,
                } as u64;
            }
        }
    }

    println!("{}", r);
}
