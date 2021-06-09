use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n.to_string().chars().any(|c| c == '9') {
        println!("Yes");
    } else {
        println!("No");
    }
}
