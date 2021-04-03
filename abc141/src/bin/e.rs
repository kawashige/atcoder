use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut l = 0;
    let mut r = n / 2 + 1;

    while l + 1 < r {
        let m = (l + r) / 2;

        if (0..(n - m)).any(|i| {
            s[(i + m)..]
                .iter()
                .collect::<String>()
                .contains(&s[i..(i + m)].iter().collect::<String>())
        }) {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
