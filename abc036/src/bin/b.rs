use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut r = vec![vec!['o'; n]; n];
    for j in 0..n {
        for i in 0..n {
            r[j][n - 1 - i] = s[i][j];
        }
    }

    for row in r {
        println!("{}", row.into_iter().collect::<String>());
    }
}
