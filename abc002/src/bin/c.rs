use proconio::input;

fn main() {
    input! {
        a: (i32, i32),
        b: (i32, i32),
        c: (i32, i32),
    }

    let bb = (b.0 - a.0, b.1 - a.1);
    let cc = (c.0 - a.0, c.1 - a.1);

    let area = (bb.0 * cc.1 - bb.1 * cc.0).abs() as f64 / 2.0;

    println!("{}", area);
}
