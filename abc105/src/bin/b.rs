use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 0..=(n / 4) {
        if (n - 4 * i) % 7 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
