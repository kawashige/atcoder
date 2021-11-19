use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut count = 0;
    for i in 0..n {
        if let Some(j) = (0..s[i].len()).rev().find(|j| s[i][*j] == '.') {
            count += 1;
            if i < n - 1 {
                for k in j..s[i + 1].len() {
                    s[i + 1][k] = 'o';
                }
            }
        }
    }

    println!("{}", count);
}
