use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize
    }

    let c = h.max(w);

    println!("{}", (n + c - 1) / c);
}
