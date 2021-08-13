use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    println!("{}", 2 * format!("{}{}", a, b).parse::<usize>().unwrap());
}
