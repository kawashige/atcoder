use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let wild = "atcoder".chars().collect::<Vec<_>>();

    for (c_s, c_t) in s.into_iter().zip(t.into_iter()) {
        if c_s == c_t || c_s == '@' && wild.contains(&c_t) || c_t == '@' && wild.contains(&c_s) {
            continue;
        }

        println!("You will lose");
        return;
    }

    println!("You can win");
}
