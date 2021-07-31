use proconio::input;

fn main() {
    input! {
        s: String
    }

    println!("{}", s.split('+').filter(|t| !t.contains('0')).count());
}
