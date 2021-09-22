use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    let x = if x > 3 { 0 } else { 4 - x };
    let y = if y > 3 { 0 } else { 4 - y };

    let prize = (x + y) * 100000 + if x == 3 && y == 3 { 400000 } else { 0 };

    println!("{}", prize);
}
