use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: i32,
        y: i32,
        mut xx: [i32; n],
        mut yy: [i32; m]
    }

    xx.sort_unstable();
    yy.sort_unstable();

    if ((x + 1)..=y).any(|i| xx[n - 1] < i && i <= yy[0]) {
        println!("No War");
    } else {
        println!("War");
    }
}
