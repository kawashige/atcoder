use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut l_black = vec![0; n];
    let mut r_white = vec![0; n];
    for i in 0..n {
        if i > 0 {
            l_black[i] = l_black[i - 1];
        }
        if s[i] == '#' {
            l_black[i] += 1;
        }
    }
    for i in (0..n).rev() {
        if i < n - 1 {
            r_white[i] = r_white[i + 1];
        }
        if s[i] == '.' {
            r_white[i] += 1;
        }
    }

    if let Some(j) = (0..n).rev().find(|i| s[*i] == '.') {
        let mut min = l_black[j];
        for i in 0..j {
            if s[i] == '#' {
                min = std::cmp::min(min, if i > 0 { l_black[i - 1] } else { 0 } + r_white[i]);
            }
        }
        println!("{}", min);
    } else {
        println!("0");
    }
}
