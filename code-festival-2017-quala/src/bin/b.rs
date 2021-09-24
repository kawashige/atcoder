use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }

    for i in 0..=n {
        for j in 0..m {
            if k == (m - j) * i + (n - i) * j {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
