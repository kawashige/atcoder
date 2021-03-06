use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    for i in 0..=x {
        if 2 * i + 4 * (x - i) == y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
