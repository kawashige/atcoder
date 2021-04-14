use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        x: usize,
        y: usize
    }

    println!(
        "{} {}",
        (w * h) as f64 / 2.0,
        if w % 2 == 0 && h % 2 == 0 && w / 2 == x && h / 2 == y {
            1
        } else {
            0
        }
    );
}
