use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let chars = s.clone().into_iter().collect::<Vec<_>>();
    let mut r = s.len() - 1;

    for c in chars {
        let mut count = 0;
        for i in 0..s.len() {
            let c = if let Some(j) = (i..s.len()).find(|j| s[*j] == c) {
                j - i
            } else {
                s.len() - i
            };
            count = count.max(c);
        }
        r = r.min(count);
    }

    println!("{}", r);
}
