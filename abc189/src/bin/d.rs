use proconio::input;

fn main() {
    input! {
        n: usize,
        strings: [String; n]
    }

    let mut count: (usize, usize) = (1, 1);
    for s in strings {
        if s == "AND" {
            count = (count.0, count.0 + 2 * count.1);
        } else {
            count = (2 * count.0 + count.1, count.1);
        }
    }
    println!("{}", count.0);
}
