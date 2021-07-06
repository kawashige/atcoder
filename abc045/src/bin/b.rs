use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
        c: Chars,
    }

    let p = ['A', 'B', 'C'];
    let mut v = Vec::with_capacity(3);
    v.push(a.into_iter().rev().collect::<Vec<char>>());
    v.push(b.into_iter().rev().collect::<Vec<char>>());
    v.push(c.into_iter().rev().collect::<Vec<char>>());

    let mut i = 0;
    loop {
        if v[i].is_empty() {
            print!("{}", p[i]);
            return;
        }
        i = v[i].pop().unwrap() as usize - 0x61;
    }
}
