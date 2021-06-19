use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let c = (108 * n) / 100;

    if c == 206 {
        println!("so-so")
    } else if c < 206 {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
