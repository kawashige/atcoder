use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    if x == y {
        println!("{}", x);
    } else if (x == 0 && y == 1) || (x == 1 && y == 0) {
        println!("2");
    } else if (x == 1 && y == 2) || (x == 2 && y == 1) {
        println!("0");
    } else if (x == 0 && y == 2) || (x == 2 && y == 0) {
        println!("1");
    }
}
