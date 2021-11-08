use proconio::input;

fn main() {
    input! {
        x: f64
    }

    println!("{}", (x + 0.5) as usize);
}
