use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64
    }

    let r = (h * w) / 2;
    if h == 1 || w == 1 {
        println!("{}", 1);
    } else if h % 2 == 0 || w % 2 == 0 {
        println!("{}", r);
    } else {
        println!("{}", r + 1);
    }
}
