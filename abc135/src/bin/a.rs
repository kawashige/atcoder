use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a < b {
        if (b - a) % 2 == 0 {
            println!("{}", a + (b - a) / 2);
        } else {
            println!("IMPOSSIBLE");
        }
    } else {
        if (a - b) % 2 == 0 {
            println!("{}", b + (a - b) / 2);
        } else {
            println!("IMPOSSIBLE");
        }
    }
}
