use proconio::input;

fn main() {
    input! {
        a: u128,
        b: String
    }

    let r = (a * b.replace(".", "").parse::<u128>().unwrap()) / 100;
    println!("{}", r);
}
