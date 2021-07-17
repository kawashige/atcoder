use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        k: usize
    }

    let c = s[k - 1];
    println!(
        "{}",
        s.into_iter()
            .map(|x| if c != x { '*' } else { x })
            .collect::<String>()
    );
}
