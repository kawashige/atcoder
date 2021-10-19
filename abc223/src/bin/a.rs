use proconio::input;

fn main() {
    input! {
        x: usize
    }

    if x >= 100 && x % 100 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
