use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 1..10 {
        for j in 1..10 {
            if i * j == 2025 - n {
                println!("{} x {}", i, j);
            }
        }
    }
}
