use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }

    println!("{}", t.iter().min().unwrap());
}
