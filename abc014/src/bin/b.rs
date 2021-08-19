use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let mut r = 0;
    for i in 0..20 {
        if x & 1 << i > 0 {
            r += a[i];
        }
    }

    println!("{}", r);
}
