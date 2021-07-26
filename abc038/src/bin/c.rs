use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut l = 1;
    let mut r: u64 = 0;
    for i in 1..n {
        if a[i - 1] < a[i] {
            l += 1;
        } else {
            r += l * (l + 1) / 2;
            l = 1;
        }
    }
    r += l * (l + 1) / 2;

    println!("{}", r);
}
