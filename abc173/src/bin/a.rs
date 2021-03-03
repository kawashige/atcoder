use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", if n % 1000 == 0 { 0 } else { 1000 - n % 1000 });
}
