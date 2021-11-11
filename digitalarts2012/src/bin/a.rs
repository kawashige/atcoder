use proconio::input;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let s = read::<String>()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    input! {
        n: usize,
        t: [String; n]
    }

    let mut r = Vec::new();

    for x in s {
        if t.iter().any(|y| {
            y.len() == x.len()
                && y.as_bytes()
                    .iter()
                    .zip(x.as_bytes().iter())
                    .all(|(a, b)| a == &b'*' || a == b)
        }) {
            r.push(std::iter::repeat('*').take(x.len()).collect::<String>());
        } else {
            r.push(x);
        }
    }

    println!("{}", r.join(" "));
}
