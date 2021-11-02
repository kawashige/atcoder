use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        chars: Chars
    }

    let buttons = ['A', 'B', 'X', 'Y'];
    let mut r = std::usize::MAX;
    for a in buttons.iter() {
        for b in buttons.iter() {
            for c in buttons.iter() {
                for d in buttons.iter() {
                    let mut i = 0;
                    let mut count = 0;
                    while i < chars.len() {
                        count += 1;
                        if i < chars.len() - 1
                            && ((&chars[i] == a && &chars[i + 1] == b)
                                || (&chars[i] == c && &chars[i + 1] == d))
                        {
                            i += 2;
                        } else {
                            i += 1;
                        }
                    }
                    r = r.min(count);
                }
            }
        }
    }

    println!("{}", r);
}
