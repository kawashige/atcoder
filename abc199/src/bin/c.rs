use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q]
    }

    let mut strings = vec![s[..n].to_vec(), s[n..].to_vec()];
    let mut first = 0;

    for (t, a, b) in tab {
        if t == 1 {
            if a <= n && b <= n {
                strings[first].swap(a - 1, b - 1);
            } else if a > n && b > n {
                strings[(first + 1) % 2].swap(a - 1 - n, b - 1 - n);
            } else if a <= n && b > n {
                let tmp = strings[first][a - 1];
                strings[first][a - 1] = strings[(first + 1) % 2][b - 1 - n];
                strings[(first + 1) % 2][b - 1 - n] = tmp;
            } else {
                let tmp = strings[first][b - 1];
                strings[first][b - 1] = strings[(first + 1) % 2][a - 1 - n];
                strings[(first + 1) % 2][a - 1 - n] = tmp;
            }
        } else {
            first = (first + 1) % 2;
        }
    }

    println!(
        "{}",
        strings[first]
            .iter()
            .chain(strings[(first + 1) % 2].iter())
            .collect::<String>()
    )
}
