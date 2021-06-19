use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    }

    let mut primes = vec![true; r + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=((r as f64).sqrt() as usize) {
        if !primes[i] {
            continue;
        }

        for j in ((i + i)..primes.len()).step_by(i) {
            primes[j] = false;
        }
    }

    let mut spf = (0..=r).collect::<Vec<usize>>();
    for i in 2..=r {
        if spf[i] == i {
            for j in ((i * 2)..=r).step_by(i) {
                spf[j] = i;
            }
        }
    }
    // println!("primes: {:?}", primes);
    // println!("spf: {:?}", spf);

    let mut count = 0;
    for i in 2..=(r / 2) {
        if primes[i] {
            let x = (r / i) - (if i < l { l - 1 } else { i }) / i;
            count += (x - 1) * x;
        } else if primes[i / spf[i]] {
            let x = r / i;
            count -= (x - 1) * x;
        }

        println!("i: {}, count: {}", i, count);
    }

    println!("{}", count);
}
