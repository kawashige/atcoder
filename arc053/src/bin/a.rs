use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }

    println!("{}", (h - 1) * w + h * (w - 1));
}
