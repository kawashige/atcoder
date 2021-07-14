use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    println!(
        "{}",
        a.into_iter()
            .enumerate()
            .filter(|(i, x)| i % 2 == 0 && x % 2 == 1)
            .count()
    );
}
