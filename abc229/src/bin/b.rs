use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    let (x, y) = if a.len() > b.len() { (a, b) } else { (b, a) };

    for (i, j) in x.chars().rev().zip(y.chars().rev()) {
        if i.to_digit(10).unwrap() + j.to_digit(10).unwrap() > 9 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
