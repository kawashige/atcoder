use proconio::input;

fn main() {
    input! {
        x: i32
    }

    println!("{}", std::cmp::max(x, 0));
}
