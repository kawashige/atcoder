use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [usize; n]
    }

    for i in 2..n {
        if t[i - 2] + t[i - 1] + t[i] < k {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
