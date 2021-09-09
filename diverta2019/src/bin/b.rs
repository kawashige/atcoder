use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        n: usize
    }

    let mut count = 0;

    for i in 0..=(n / r) {
        for j in 0..=((n - i * r) / g) {
            if (n - i * r - j * g) % b == 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
