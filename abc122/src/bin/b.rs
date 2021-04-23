use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let l = s
        .into_iter()
        .fold((0, 0), |(l, count), c| {
            if ['A', 'C', 'G', 'T'].contains(&c) {
                (l + 1, std::cmp::max(l + 1, count))
            } else {
                (0, count)
            }
        })
        .1;

    println!("{}", l);
}
