use proconio::input;

fn main() {
    input! {
        s: String
    }

    println!("{} {}", &s[..4], &s[4..]);
}
