use proconio::input;

fn main() {
    input! {
        s: String
    }

    if s.starts_with("YAKI") {
        println!("Yes");
    } else {
        println!("No");
    }
}
