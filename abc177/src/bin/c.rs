use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u128; n]
    }

    let m = 1_000_000_007;
    let mut sums = vec![0; n];
    sums[n - 1] = a[n - 1];
    for i in (0..(n - 1)).rev() {
        sums[i] = (sums[i + 1] + a[i]) % m;
    }

    let mut sum = 0;
    for i in 0..(n - 1) {
        sum += a[i] * sums[i + 1] % m;
        sum %= m;
    }
    println!("{}", sum);
}
