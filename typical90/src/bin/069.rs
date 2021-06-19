use proconio::input;

fn mod_pow(x: u64, n: u64) -> u64 {
    let m = 1_000_000_007;
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = mod_pow(x, n / 2);
        r * r % m
    } else {
        x * mod_pow(x, n - 1) % m
    }
}

fn main() {
    input! {
        n: u64,
        k: u64
    }

    let m = 1_000_000_007;

    if (n == 2 && k == 1) || (n >= 3 && k < 3) {
        println!("0");
        return;
    }

    let mut r = k;
    if n > 1 {
        r = r * (k - 1) % m;
        r = r * mod_pow(k - 2, n - 2) % m;
    }

    println!("{}", r);
}
