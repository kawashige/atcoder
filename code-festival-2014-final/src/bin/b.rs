use proconio::input;

fn main() {
    input! {
        s: String
    }

    let x = s
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + if i % 2 == 0 { x } else { -x });

    println!("{}", x);
}
