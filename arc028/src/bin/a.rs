use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    if n % (a + b) <= a && n % (a + b) != 0 {
        println!("Ant");
    } else {
        println!("Bug");
    }
}
