use proconio::input;

fn main() {
    input! {
        s: char,
        t: String
    }

    if s == 'Y' {
        println!("{}", t.to_ascii_uppercase());
    } else {
        println!("{}", t);
    }
}
