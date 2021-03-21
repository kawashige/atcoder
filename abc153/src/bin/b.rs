use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n]
    }

    if h <= a.iter().sum() {
        println!("Yes");
    } else {
        println!("No");
    }
}
