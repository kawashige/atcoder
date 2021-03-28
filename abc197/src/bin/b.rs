use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
    }

    let mut count = 1;

    count += (x..h).take_while(|i| s[*i][y - 1] == '.').count();
    count += (0..(x - 1))
        .rev()
        .take_while(|i| s[*i][y - 1] == '.')
        .count();
    count += (y..w).take_while(|i| s[x - 1][*i] == '.').count();
    count += (0..(y - 1))
        .rev()
        .take_while(|i| s[x - 1][*i] == '.')
        .count();

    println!("{}", count);
}
