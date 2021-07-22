use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    }

    ab.sort_unstable();

    println!("{}", ab[n - 1].0 + ab[n - 1].1);
}
