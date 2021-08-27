use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    for i in 0..=n {
        if m + i >= 3 * n && 4 * n >= m + 2 * i {
            println!("{} {} {}", i, 4 * n - 2 * i - m, m + i - 3 * n);
            return;
        }
    }

    println!("-1 -1 -1");
}
