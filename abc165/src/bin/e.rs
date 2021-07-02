use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize
    }

    let mut s = 1;
    let mut e = 1 + m;

    while s < e {
        println!("{} {}", s, e);
        s += 1;
        e -= 1;
    }

    let mut s = 2 + m;
    let mut e = s + m - 1;

    while s < e {
        println!("{} {}", s, e);
        s += 1;
        e -= 1;
    }
}
