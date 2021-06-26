use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64
    }

    if x == y || (x - y).abs() == 1 {
        print!("Brown");
    } else {
        print!("Alice");
    }
}
