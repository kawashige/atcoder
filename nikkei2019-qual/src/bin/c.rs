use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n]
    }

    ab.sort_unstable_by_key(|(a, b)| a + b);
    let mut r = 0;

    for i in (0..n).rev() {
        if (n - 1 - i) % 2 == 0 {
            r += ab[i].0
        } else {
            r -= ab[i].1
        }
    }

    println!("{}", r);
}
