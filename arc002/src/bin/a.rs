use proconio::input;

fn main() {
    input! {
        y: usize
    }

    if y % 400 == 0 || (y % 4 == 0 && y % 100 != 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}
