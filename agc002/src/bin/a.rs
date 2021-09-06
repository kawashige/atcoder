use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    if a <= 0 && 0 <= b {
        println!("Zero");
    } else if a < 0 && b < 0 {
        if (a.abs() - b.abs() + 1) % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    } else if a < 0 && 0 < b {
        if a.abs() % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    } else {
        println!("Positive");
    }
}
