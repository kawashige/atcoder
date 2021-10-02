use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    if a.abs() == b.abs() {
        println!("Draw");
    } else if a.abs() < b.abs() {
        println!("Ant");
    } else {
        println!("Bug");
    }
}
