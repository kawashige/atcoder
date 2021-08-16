use proconio::input;

fn main() {
    input! {
        se: [(usize, usize); 3]
    }

    println!("{}", se.into_iter().map(|(s, e)| s * e / 10).sum::<usize>());
}
