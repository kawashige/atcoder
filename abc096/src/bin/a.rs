use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let c = if a <= b { a } else { a - 1 };
    println!("{}", c);
}
