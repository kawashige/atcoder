use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a <= 8 && b <= 8 {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
