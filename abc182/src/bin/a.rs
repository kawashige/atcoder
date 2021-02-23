use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    println!("{}", std::cmp::max(0, 2 * a + 100 - b))
}
