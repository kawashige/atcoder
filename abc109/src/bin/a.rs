use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a == 2 || b == 2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
