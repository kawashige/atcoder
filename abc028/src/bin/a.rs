use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n < 60 {
        println!("Bad");
    } else if n < 90 {
        println!("Good");
    } else if n < 100 {
        println!("Great");
    } else {
        println!("Perfect");
    }
}
