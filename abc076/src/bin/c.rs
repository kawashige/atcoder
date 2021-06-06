use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }

    let mut found = false;
    for i in (0..=(s.len() - t.len())).rev() {
        if (0..t.len()).all(|j| s[i + j] == '?' || s[i + j] == t[j]) {
            found = true;
            (0..t.len()).for_each(|j| s[i + j] = t[j]);
            (0..s.len()).for_each(|j| {
                if s[j] == '?' {
                    s[j] = 'a'
                }
            });
            break;
        }
    }

    if found {
        println!("{}", s.into_iter().collect::<String>());
    } else {
        println!("UNRESTORABLE");
    }
}
