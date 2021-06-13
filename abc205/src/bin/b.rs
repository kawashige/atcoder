use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    for i in 0..n {
        if a[i] != i + 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
