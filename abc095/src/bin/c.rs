use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
        y: usize
    }

    let min = std::cmp::min(x, y);

    let mut cost = min * if a + b >= 2 * c { 2 * c } else { a + b };

    if x > y {
        cost += (x - min) * if a > 2 * c { 2 * c } else { a }
    } else if x < y {
        cost += (y - min) * if b > 2 * c { 2 * c } else { b }
    }

    println!("{}", cost)
}
