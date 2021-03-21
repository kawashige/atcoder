use proconio::input;

fn recurse(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        1 + 2 * recurse(n / 2)
    }
}

fn main() {
    input! {
        h: usize
    }

    println!("{}", recurse(h));
}
