use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ac: [(usize, usize); m]
    }

    ac.sort_unstable_by(|a, b| a.1.cmp(&b.1));
}
