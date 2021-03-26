use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a != 1 && b != 1 {
        println!("1")
    } else if a != 2 && b != 2 {
        println!("2")
    } else if a != 3 && b != 3 {
        println!("3")
    }
}
