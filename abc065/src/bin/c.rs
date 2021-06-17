use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32
    }

    if (n - m).abs() > 1 {
        println!("0");
        return;
    }

    let mut factorial = vec![0; std::cmp::max(m, n) as usize + 1];
    factorial[0] = 1;
    factorial[1] = 1;
    for i in 2..factorial.len() {
        factorial[i] = factorial[i - 1] * i % 1_000_000_007;
    }

    let mut r = factorial[m as usize] * factorial[n as usize] % 1_000_000_007;
    if n == m {
        r = (r + r) % 1_000_000_007;
    }

    println!("{}", r);
}
