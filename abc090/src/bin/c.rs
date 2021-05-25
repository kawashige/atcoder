use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    if n == 1 && m == 1 {
        println!("1")
    } else if n == 1 {
        println!("{}", m - 2);
    } else if m == 1 {
        println!("{}", n - 2);
    } else {
        println!("{}", (n - 2) as u64 * (m - 2) as u64);
    }
}
