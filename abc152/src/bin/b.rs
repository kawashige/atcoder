use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a <= b {
        println!("{}", a.to_string().repeat(b));
    } else {
        println!("{}", b.to_string().repeat(a));
    }
}
