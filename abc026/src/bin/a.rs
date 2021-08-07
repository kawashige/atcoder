use proconio::input;

fn main() {
    input! {
        a: usize
    }

    println!("{}", (1..a).map(|x| x * (a - x)).max().unwrap());
}
