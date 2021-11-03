use proconio::input;

fn main() {
    input! {
        h: f64,
        bmi: f64
    }

    println!("{}", bmi * (h / 100.0) * (h / 100.0));
}
