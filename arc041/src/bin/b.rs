use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [Chars; n]
    }

    let mut b = b
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut a = vec![vec![0; m]; n];

    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            a[i][j] = b[i - 1][j]
                .min(b[i][j - 1])
                .min(b[i][j + 1])
                .min(b[i + 1][j]);
            b[i - 1][j] -= a[i][j];
            b[i][j - 1] -= a[i][j];
            b[i][j + 1] -= a[i][j];
            b[i + 1][j] -= a[i][j];
        }
    }

    for row in a {
        println!(
            "{}",
            row.into_iter().map(|x| x.to_string()).collect::<String>()
        );
    }
}
