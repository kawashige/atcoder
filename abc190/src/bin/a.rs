use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    if b < a || a == b && c == 1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
