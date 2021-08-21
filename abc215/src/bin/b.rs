use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut k = 0;
    let mut x = 1;

    while x * 2 <= n {
        x *= 2;
        k += 1;
    }

    println!("{}", k);
}
