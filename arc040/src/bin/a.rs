use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let (takahashi, aoki) = s
        .into_iter()
        .flatten()
        .fold((0, 0), |(takahashi, aoki), c| match c {
            'R' => (takahashi + 1, aoki),
            'B' => (takahashi, aoki + 1),
            _ => (takahashi, aoki),
        });

    if takahashi == aoki {
        println!("DRAW")
    } else if takahashi > aoki {
        println!("TAKAHASHI")
    } else {
        println!("AOKI")
    }
}
