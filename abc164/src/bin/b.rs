use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }

    if (a + d - 1) / d >= (c + b - 1) / b {
        println!("Yes");
    } else {
        println!("No");
    }
}
