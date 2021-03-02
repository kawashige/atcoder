use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i128,
        points: [(i128, i128); n]
    }

    let count = points
        .iter()
        .filter(|(x, y)| x * x + y * y <= d * d)
        .count();
    println!("{}", count);
}
