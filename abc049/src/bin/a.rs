use proconio::input;

fn main() {
    input! {
        c: char
    }

    if "aeiou".chars().any(|i| i == c) {
        println!("vowel");
    } else {
        println!("consonant");
    }
}
