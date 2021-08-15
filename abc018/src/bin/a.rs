use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let mut v = vec![a, b, c];
    v.sort_unstable();

    println!("{}", 3 - v.binary_search(&a).unwrap());
    println!("{}", 3 - v.binary_search(&b).unwrap());
    println!("{}", 3 - v.binary_search(&c).unwrap());
}
