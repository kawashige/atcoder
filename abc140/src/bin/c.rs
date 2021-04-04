use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n - 1]
    }

    let sum: usize = b[0]
        + b.last().unwrap()
        + b.windows(2)
            .map(|v| std::cmp::min(v[0], v[1]))
            .sum::<usize>();

    println!("{}", sum);
}
