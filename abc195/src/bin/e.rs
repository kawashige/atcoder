use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars
    }

    let mut takahashi = 1;
    for (d, p) in s.into_iter().rev().zip(t.into_iter().rev()) {
        let mut new_takahashi = 0;
        let d = d.to_digit(10).unwrap() as usize;
        for i in 0..7 {
            let n1 = (i * 10 + d) % 7;
            let n2 = (i * 10) % 7;

            if p == 'T' {
                if takahashi & 1 << n1 > 0 || takahashi & 1 << n2 > 0 {
                    new_takahashi |= 1 << i;
                }
            } else {
                if takahashi & 1 << n1 > 0 && takahashi & 1 << n2 > 0 {
                    new_takahashi |= 1 << i;
                }
            }
        }
        if new_takahashi == 0 {
            println!("Aoki");
            return;
        }
        takahashi = new_takahashi;
    }

    if takahashi & 1 == 1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
