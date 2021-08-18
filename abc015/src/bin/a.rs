use proconio::input;

fn main() {
    input! {
        a: String,
        b: String
    }

    if a.len() < b.len() {
        println!("{}", b);
    } else {
        println!("{}", a);
    }
}
