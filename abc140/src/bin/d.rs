use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut k = k;
    let mut lr = 0;
    let mut rl = 0;
    let mut count = 0;
    for i in 1..n {
        if s[i - 1] == s[i] {
            count += 1;
        } else if s[i - 1] == 'L' && s[i] == 'R' {
            lr += 1;
        } else {
            rl += 1;
        }
        if lr > 0 && rl > 0 && k > 0 {
            k -= 1;
            count += 2;
            let tmp = rl;
            rl = lr - 1;
            lr = tmp - 1;
        }
    }

    if (lr > 0 || rl > 0) && k > 0 {
        count += 1;
    }

    println!("{}", count);
}
