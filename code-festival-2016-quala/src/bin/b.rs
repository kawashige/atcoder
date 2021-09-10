use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut r = 0;
    for i in 0..n {
        if i + 1 == a[a[i] - 1] {
            r += 1;
        }
    }

    println!("{}", r / 2);
}
