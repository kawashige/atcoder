use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: String,
    }

    let mut r = String::new();
    for c in t.chars() {
        match c {
            '1' => r += &s1,
            '2' => r += &s2,
            _ => r += &s3,
        }
    }

    println!("{}", r);
}
