use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    for i in k..n {
        if a[i - k] < a[i] {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
