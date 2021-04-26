use proconio::input;

fn main() {
    input! {
        n: usize,
        xu: [(String, String); n]
    }

    let sum = xu
        .into_iter()
        .map(|(x, u)| {
            if u == "JPY" {
                x.parse::<f64>().unwrap()
            } else {
                x.parse::<f64>().unwrap() * 380000.0
            }
        })
        .sum::<f64>();

    println!("{}", sum);
}
