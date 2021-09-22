use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let len = a.iter().sum::<i64>();
    let mut left = 0;

    let mut min = std::i64::MAX;
    for i in 0..(n - 1) {
        left += a[i];
        min = min.min((len - left - left).abs())
    }

    println!("{}", min);
}
