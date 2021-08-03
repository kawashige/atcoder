use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if b * c == a * d {
        println!("DRAW");
    } else if b * c < a * d {
        println!("AOKI")
    } else {
        println!("TAKAHASHI")
    }
}
