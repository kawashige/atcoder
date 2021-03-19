use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n]
    }

    let mut min = std::i32::MAX;
    for i in 1..=100 {
        min = std::cmp::min(min, x.iter().map(|j| (j - i).pow(2)).sum::<i32>());
    }

    println!("{}", min);
}
