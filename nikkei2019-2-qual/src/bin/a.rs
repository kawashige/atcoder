use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let r = if n % 2 == 0 { n - 1 } else { n } / 2;

    println!("{}", r);
}
