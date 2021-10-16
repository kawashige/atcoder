use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let r = (ab.into_iter().map(|(a, b)| a * b).sum::<usize>() * 105) / 100;

    println!("{}", r);
}
