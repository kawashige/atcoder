use proconio::input;

fn main() {
    input! {
        h: usize,
        a: usize,
    }

    println!("{}", (h + a - 1) / a);
}
