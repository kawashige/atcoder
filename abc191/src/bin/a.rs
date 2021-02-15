use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }

    if ((v * t)..=(s * v)).contains(&d) {
        println!("No");
    } else {
        println!("Yes");
    }
}
