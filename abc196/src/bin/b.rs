use proconio::input;

fn main() {
    input! {
        x: String
    }

    println!("{}", x.split(".").next().unwrap());
}
