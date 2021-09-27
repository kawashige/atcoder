use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64
    }

    let sum = a.iter().sum::<u64>();

    let mut r = x / sum * (n as u64);
    let mut target = x - x / sum * sum;

    for i in 0..n {
        if target < a[i] {
            r += i as u64 + 1;
            break;
        }
        target -= a[i];
    }

    println!("{}", r);
}
