use proconio::input;

fn main() {
    input! {
        x: [String; 5]
    }

    println!("{}", (1_usize..6).find(|i| x[i - 1] == "0").unwrap());
}
