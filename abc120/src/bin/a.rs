use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if b / a < c {
        println!("{}", b / a);
    } else {
        println!("{}", c);
    }
}
