use proconio::input;

fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize
    }

    if t * s >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
