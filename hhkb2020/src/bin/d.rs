use proconio::input;

fn main() {
    input! {
        t: usize
    }

    const M: i64 = 1_000_000_007;

    for _ in 0..t {
        input! {
            n: i64,
            mut a: i64,
            mut b: i64
        }

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        if a + b > n {
            println!("0")
        } else {
            let mut count = (n - a - b + 1) * (n - a + 1);
            count %= M;
            count *= n - b + 1;
            count %= M;
            count *= 2;
            count %= M;
            count -= ((n - a - b + 1) * (n - a - b + 1) % M) * 4 % M;
            if count < 0 {
                count += M;
            }

            println!("{}", count);
        }
    }
}
