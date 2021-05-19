use proconio::input;

fn main() {
    input! {
        n: usize
    }

    const MAX: usize = 55555;

    let mut primes = vec![true; MAX + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=((MAX as f64).sqrt() as usize) {
        if !primes[i] {
            continue;
        }

        for j in ((i * 2)..=MAX).step_by(i) {
            primes[j] = false;
        }
    }

    let mut r = Vec::with_capacity(n);
    for i in 0..=MAX {
        if primes[i] && i % 10 == 1 {
            r.push(i);
            if r.len() == n {
                break;
            }
        }
    }
    println!(
        "{}",
        r.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
