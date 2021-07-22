use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, u64); n]
    }

    let mut x = 0;
    for i in 0..=(0..30)
        .rev()
        .find(|i| k & (1 << i) as usize > 0)
        .unwrap_or(0)
    {
        x |= 1 << i;
    }

    let mut r: u64 = 0;
    for (a, b) in &ab {
        if a | k == k {
            r += b;
        }
    }

    for i in (0..30).rev() {
        if k & 1 << i > 0 {
            let x2 = x & !(1 << i);

            let mut sum = 0;
            for (a, b) in &ab {
                if a | x2 == x2 {
                    sum += b;
                }
            }
            r = std::cmp::max(r, sum);
        } else {
            x &= !(1 << i);
        }
    }

    println!("{}", r);
}
