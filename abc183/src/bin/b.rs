use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64
    }

    let d = (sy + gy) / (gx - sx);
    println!("{:.10}", sx + sy / d);
}
