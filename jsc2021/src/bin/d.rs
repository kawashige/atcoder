use proconio::input;

fn mod_pow(x: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = mod_pow(x, n / 2);
        r * r % 1_000_000_007
    } else {
        x * mod_pow(x, n - 1) % 1_000_000_007
    }
}

fn main() {
    input! {
        n: usize,
        p: usize
    }

    let m = 1_000_000_007;

    let result = (p - 1) * mod_pow(p - 2, n - 1) % m;

    println!("{}", result);
}
