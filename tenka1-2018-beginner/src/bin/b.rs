use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        k: usize
    }

    for i in 0..k {
        if i % 2 == 0 {
            a /= 2;
            b += a;
        } else {
            b /= 2;
            a += b;
        }
    }

    println!("{} {}", a, b);
}
