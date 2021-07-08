use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        w: Chars
    }

    if w.into_iter()
        .fold(vec![0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        })
        .into_iter()
        .all(|c| c % 2 == 0)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
