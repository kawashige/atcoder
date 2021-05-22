use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let r = 21 - (a + b + c);
    println!("{}", r);
}
