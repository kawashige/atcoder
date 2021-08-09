use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n]
    }

    let mut r = t;
    for i in 1..n {
        if a[i] != a[i - 1] {
            r += (a[i] - a[i - 1]).min(t);
        }
    }

    println!("{}", r);
}
