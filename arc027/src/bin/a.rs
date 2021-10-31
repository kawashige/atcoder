use proconio::input;

fn main() {
    input! {
        h: usize,
        m: usize
    }

    println!("{}", 18 * 60 - h * 60 - m);
}
