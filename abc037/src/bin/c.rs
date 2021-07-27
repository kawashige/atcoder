use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }

    let mut sum = a[..k].iter().sum::<u64>();
    let mut r = sum;
    for i in k..n {
        sum = sum + a[i] - a[i - k];
        r += sum;
    }

    println!("{}", r);
}
