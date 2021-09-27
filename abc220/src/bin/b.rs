use proconio::input;

fn main() {
    input! {
        k: usize,
        a: String,
        b: String
    }

    let n_a = a
        .to_string()
        .chars()
        .fold(0, |acc, c| acc * k + c.to_digit(k as u32).unwrap() as usize);
    let n_b = b
        .to_string()
        .chars()
        .fold(0, |acc, c| acc * k + c.to_digit(k as u32).unwrap() as usize);

    println!("{}", n_a * n_b);
}
