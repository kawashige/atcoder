use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n]
    }

    println!(
        "{}",
        ab.into_iter().filter(|(a, b)| a >= &h && b >= &w).count()
    );
}
