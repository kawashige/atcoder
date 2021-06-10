use proconio::input;

fn main() {
    input! {
        x: usize,
        t: usize
    }

    if t > x {
        println!("0");
    } else {
        println!("{}", x - t);
    }
}
