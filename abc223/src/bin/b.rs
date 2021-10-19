use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut r = Vec::new();
    r.push(s.iter().collect::<String>());

    let mut ss = s.clone();
    for _ in 0..ss.len() {
        ss.rotate_left(1);
        r.push(ss.iter().collect::<String>());
    }

    let mut ss = s.clone();
    for _ in 0..ss.len() {
        ss.rotate_right(1);
        r.push(ss.iter().collect::<String>());
    }

    r.sort_unstable();

    println!("{}", r[0]);
    println!("{}", r.last().unwrap());
}
