use proconio::input;

fn main() {
    input! {
        s: String
    }

    let c = (0..=8).filter(|i| &s[*i..(i + 4)] == "ZONe").count();
    println!("{}", c);
}
