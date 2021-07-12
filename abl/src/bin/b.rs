use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }

    if std::cmp::max(a, c) <= std::cmp::min(b, d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
