use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut c: usize,
        mut d: usize
    }

    if c < a {
        std::mem::swap(&mut a, &mut c);
        std::mem::swap(&mut b, &mut d);
    }

    let m = if a <= c && c <= b {
        std::cmp::min(b, d) - c
    } else {
        0
    };
    println!("{}", m);
}
