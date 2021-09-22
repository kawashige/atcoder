use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        _k: usize,
        s: [Chars; h]
    }

    let mut cut = vec![vec![0; w]; h];

    let mut no = 1;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                cut[i][j] = no;
                no += 1;
            }
        }
    }

    for i in 0..h {
        for j in 1..w {
            if cut[i][j] == 0 && cut[i][j - 1] > 0 {
                cut[i][j] = cut[i][j - 1];
            }
        }
    }

    for i in 0..h {
        for j in (0..(w - 1)).rev() {
            if cut[i][j] == 0 && cut[i][j + 1] > 0 {
                cut[i][j] = cut[i][j + 1];
            }
        }
    }

    for i in 1..h {
        for j in 0..w {
            if cut[i][j] == 0 && cut[i - 1][j] > 0 {
                cut[i][j] = cut[i - 1][j];
            }
        }
    }

    for i in (0..(h - 1)).rev() {
        for j in 0..w {
            if cut[i][j] == 0 && cut[i + 1][j] > 0 {
                cut[i][j] = cut[i + 1][j];
            }
        }
    }

    for row in cut {
        println!(
            "{}",
            row.into_iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
