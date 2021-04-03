use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    let max = (std::cmp::min(a, b) as f64).sqrt() as usize;
    let mut count = 1;
    let mut primes = vec![true; max + 1];
    for i in 2..=max {
        if !primes[i] {
            continue;
        }

        for j in ((i * 2)..=max).step_by(i) {
            primes[j] = false;
        }

        if a % i == 0 && b % i == 0 {
            count += 1;
        }
        while a % i == 0 {
            a /= i;
        }
        while b % i == 0 {
            b /= i;
        }
        if a < i || b < i {
            break;
        }
    }
    if a == b && a != 1 {
        count += 1;
    }

    println!("{}", count);
}
