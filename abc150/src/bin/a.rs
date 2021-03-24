use proconio::input;

fn main() {
    input! {
        k: usize,
        x: usize
    }

    if 500 * k >= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
