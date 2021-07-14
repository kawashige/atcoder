use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize
    }

    println!("{}", r / d - (l - 1) / d);
}
