use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    }

    let c = lr.into_iter().map(|(l, r)| r - l + 1).sum::<usize>();

    println!("{}", c);
}
