use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }

    if d * c <= b {
        println!("-1");
    } else {
        println!("{}", (a + d * c - b - 1) / (d * c - b));
    }
}
