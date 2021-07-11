use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if i < h - 1 && s[i + 1][j] == '.' {
                count += 1;
            }
            if j < w - 1 && s[i][j + 1] == '.' {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
