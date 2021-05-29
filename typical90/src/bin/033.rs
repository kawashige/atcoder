use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let r = if h == 1 {
        w
    } else if w == 1 {
        h
    } else {
        ((w + 1) / 2) * ((h + 1) / 2)
    };

    println!("{}", r);
}
