use proconio::input;

fn main() {
    input! {
        n: i128
    }

    if n == 1 {
        println!("0");
    } else {
        let m = 1_000_000_007;
        let result = (2 * m + pow(10, n) + pow(8, n) - 2 * pow(9, n)) % m;
        println!("{}", result);
    }
}

fn pow(a: i128, n: i128) -> i128 {
    let m = 1_000_000_007;
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = pow(a, n / 2);
        r * r % m
    } else {
        a * pow(a, n - 1) % m
    }
}
