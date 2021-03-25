use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let n = 100_003;
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    let max = (n as f64).sqrt() as usize;
    for i in 2..=max {
        if primes[i] {
            for j in ((i * 2)..=n).step_by(i) {
                primes[j] = false;
            }
        }
    }

    for i in x..=n {
        if primes[i] {
            println!("{}", i);
            return;
        }
    }
}
