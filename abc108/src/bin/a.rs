use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n % 2 == 0 {
        println!("{}", (n / 2) * (n / 2));
    } else {
        println!("{}", ((n + 1) / 2) * (n - 1) / 2);
    }
}
