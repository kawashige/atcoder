use proconio::input;

fn main() {
    input! {
        n: String
    }

    if n.chars().rev().collect::<String>() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
