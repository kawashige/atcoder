use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n < 126 {
        println!("{}", 4);
    } else if n < 212 {
        println!("{}", 6);
    } else {
        println!("{}", 8);
    }
}
