use proconio::input;

fn main() {
    input! {
        x: usize
    }

    if x == 3 || x == 5 || x == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
