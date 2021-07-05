use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut count = 0;
    let mut max_ok = -1;
    let mut max_ng = -1;

    for i in 0..n {
        if s[i] == 'o' {
            count += max_ng + 1;
            max_ok = i as i64;
        } else if s[i] == 'x' {
            count += max_ok + 1;
            max_ng = i as i64;
        }
    }

    println!("{}", count);
}
