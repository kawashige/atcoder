use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i32; n]
    }

    let sum = w.iter().sum::<i32>();
    let mut s = 0;
    let mut min = std::i32::MAX;
    for i in 0..n {
        s += w[i];
        min = std::cmp::min(min, (sum - s - s).abs());
    }

    println!("{}", min);
}
