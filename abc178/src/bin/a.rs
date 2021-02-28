use proconio::input;

fn main() {
    input! {
        x: usize
    }

    println!("{}", if x == 0 { 1 } else { 0 });
}
