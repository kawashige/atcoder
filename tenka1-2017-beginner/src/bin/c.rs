use proconio::input;

fn main() {
    input! {
        n: u64
    }

    for i in 1..=3500 {
        for j in 1..=3500 {
            if 4 * i * j > n * i + n * j && (n * i * j) % (4 * i * j - n * i - n * j) == 0 {
                let k = (n * i * j) / (4 * i * j - n * i - n * j);
                if k <= 3500 {
                    println!("{} {} {}", i, j, k);
                    return;
                }
            }
        }
    }
}
