use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut r0: u64 = 0;
    let mut r1: u64 = 0;
    for i in 1..=n {
        if i % k == 0 {
            r0 += 1;
        } else if k % 2 == 0 && i % (k / 2) == 0 {
            r1 += 1;
        }
    }

    println!("{}", r0 * r0 * r0 + r1 * r1 * r1);
}
