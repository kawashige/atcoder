use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut r = vec![(s[0], 1)];
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            r.last_mut().unwrap().1 += 1;
        } else {
            r.push((s[i], 1));
        }
    }

    println!(
        "{}",
        r.into_iter()
            .map(|(c, i)| format!("{}{}", c, i))
            .collect::<String>()
    );
}
