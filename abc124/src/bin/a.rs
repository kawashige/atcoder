use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a == b {
        println!("{}", a + b)
    } else {
        let max = std::cmp::max(a, b);
        println!("{}", max + max - 1);
    }
}
