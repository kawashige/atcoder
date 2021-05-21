use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize
    }

    if a <= x && x <= a + b {
        println!("YES");
    } else {
        println!("NO");
    }
}
