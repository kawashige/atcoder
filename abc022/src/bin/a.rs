use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        t: i32,
        mut w: i32,
        a: [i32; n - 1]
    }

    let mut d = if s <= w && w <= t { 1 } else { 0 };

    for x in a {
        w += x;
        if s <= w && w <= t {
            d += 1;
        }
    }

    println!("{}", d);
}
