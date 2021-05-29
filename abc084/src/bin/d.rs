use proconio::input;

fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q]
    }

    let mut primes = vec![true; 100001];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=((100001 as f64).sqrt() as usize) {
        if !primes[i] {
            continue;
        }

        for j in ((i + i)..100001).step_by(i) {
            primes[j] = false;
        }
    }

    let mut counts = vec![0; 100001];
    for i in 1..100001 {
        counts[i] = counts[i - 1];
        if i % 2 == 1 && primes[i] && primes[(i + 1) / 2] {
            counts[i] += 1;
        }
    }

    for (l, r) in lr {
        println!("{}", counts[r] - counts[l - 1]);
    }
}
