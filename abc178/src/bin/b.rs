use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
        d: i128
    }

    println!("{}", vec![a * c, a * d, b * c, b * d].iter().max().unwrap());
}
