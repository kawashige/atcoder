use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if a == c {
        println!("{}", b);
    } else {
        println!("0");
    }
}
