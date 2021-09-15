use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }

    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut min = std::usize::MAX;

    for i in 1..2_usize.pow(16) {
        let mut num = 1;
        let mut seen = vec![false; n];
        for j in 0..15 {
            if i & 1 << j > 0 {
                num *= primes[j];
                for k in 0..n {
                    if !seen[k] && x[k] % primes[j] == 0 {
                        seen[k] = true;
                    }
                }
            }
        }

        if seen.iter().all(|s| *s) {
            min = min.min(num);
        }
    }

    println!("{}", min);
}
