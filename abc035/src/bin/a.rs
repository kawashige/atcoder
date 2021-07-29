use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize
    }

    if w % 4 == 0 && h / (w / 4) == 3 {
        println!("4:3");
    } else {
        println!("16:9");
    }
}
