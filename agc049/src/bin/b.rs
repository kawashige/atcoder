use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let mut j = 0;
    let mut count = 0;

    for i in 0..n {
        let c = if j != 0 && i < j { '0' } else { s[i] };
        if c != t[i] {
            j = j.max(i + 1);
            while j < s.len() && s[j] != '1' {
                j += 1;
            }
            if j == s.len() {
                println!("-1");
                return;
            }
            count += j - i;
            j += 1;
        }
    }

    println!("{}", count);
}
