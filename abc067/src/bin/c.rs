use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let sum = a.iter().sum::<i64>();
    let mut left_sum = 0;
    let mut min = std::i64::MAX;
    for i in 0..(n - 1) {
        left_sum += a[i];
        min = std::cmp::min(min, (sum - left_sum * 2).abs());
    }

    println!("{}", min);
}
