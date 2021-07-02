use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i32; n],
        m: usize,
        px: [(usize, i32); m]
    }

    let sum = t.iter().sum::<i32>();

    for (p, x) in px {
        println!("{}", sum + x - t[p - 1]);
    }
}
