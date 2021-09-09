use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut r = 0;

    for i in 1..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            if 2 < i && n % (i - 1) == n / (i - 1) {
                r += i - 1;
            }
            if 2 < n / i && i != n / i && n % (n / i - 1) == n / (n / i - 1) {
                r += n / i - 1;
            }
        }
    }

    println!("{}", r);
}
