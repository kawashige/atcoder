use proconio::input;

fn main() {
    input! {
        n: String
    }

    if n.contains(&"7") {
        println!("Yes");
    } else {
        println!("No");
    }
}
