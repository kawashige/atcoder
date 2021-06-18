use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();
    println!("{}", a.last().unwrap() - a[0]);
}
