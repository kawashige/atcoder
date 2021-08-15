use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        n: usize,
        lr: [(usize, usize); n]
    }

    for (l, r) in lr {
        s = s[..(l - 1)]
            .iter()
            .chain(s[(l - 1)..r].iter().rev())
            .chain(s[r..].iter())
            .cloned()
            .collect::<Vec<_>>();
    }

    println!("{}", s.into_iter().collect::<String>());
}
