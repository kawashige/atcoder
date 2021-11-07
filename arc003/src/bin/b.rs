use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n]
    }

    s.sort_unstable_by_key(|w| w.chars().rev().collect::<String>());
    for x in s {
        println!("{}", x);
    }
}
