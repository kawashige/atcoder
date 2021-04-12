use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n]
    }

    d.sort_unstable();
    if d[n / 2] == d[n / 2 - 1] {
        println!("0");
    } else {
        println!("{}", d[n / 2] - d[n / 2 - 1]);
    }
}
