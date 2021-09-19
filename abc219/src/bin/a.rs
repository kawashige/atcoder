use proconio::input;

fn main() {
    input! {
        x: usize
    }

    if x >= 90 {
        println!("expert");
    } else if x < 40 {
        println!("{}", 40 - x)
    } else if x < 70 {
        println!("{}", 70 - x)
    } else {
        println!("{}", 90 - x)
    }
}
