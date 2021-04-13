use proconio::input;

fn main() {
    input! {
        s: String
    }

    let s = s.trim_end_matches("0");
    if s == s.chars().rev().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
