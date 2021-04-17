use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    for i in (1..=(b - a)).rev() {
        let s = i * ((a + i - 1) / i);
        if s + i <= b {
            println!("{}", i);
            return;
        }
    }
}
