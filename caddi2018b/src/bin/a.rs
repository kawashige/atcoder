use proconio::input;

fn main() {
    input! {
        n: String
    }

    println!("{}", n.chars().filter(|c| c == &'2').count());
}
