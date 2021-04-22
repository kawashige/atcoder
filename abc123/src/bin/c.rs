use proconio::input;

fn main() {
    input! {
        n: u64,
        v: [u64; 5]
    }

    let min = v.iter().min().unwrap();
    let r = 4 + (n + min - 1) / min;

    println!("{}", r);
}
