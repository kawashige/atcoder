use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize
    }

    println!("{}", (n - h + 1) * (n - w + 1));
}
