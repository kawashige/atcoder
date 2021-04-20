use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize
    }

    let mut count = 0;
    let mut time = a;
    while time <= t {
        count += b;
        time += a;
    }

    println!("{}", count);
}
