use proconio::input;

fn main() {
    input! {
        n: usize,
        _l: usize,
        mut s: [String; n]
    }

    s.sort_unstable();
    println!("{}", s.into_iter().collect::<String>());
}
