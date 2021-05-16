use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n < 1000 {
        println!("ABC");
    } else {
        println!("ABD");
    }
}
