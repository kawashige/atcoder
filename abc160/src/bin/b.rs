use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let n = (x / 500) * 1000 + ((x % 500) / 5) * 5;
    println!("{}", n);
}
