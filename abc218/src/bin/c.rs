use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let min_h = (0..n).find(|i| (0..n).any(|j| s[*i][j] == '#')).unwrap();
    let max_h = (0..n)
        .rev()
        .find(|i| (0..n).any(|j| s[*i][j] == '#'))
        .unwrap();
    let min_w = (0..n).find(|i| (0..n).any(|j| s[j][*i] == '#')).unwrap();
    let max_w = (0..n)
        .rev()
        .find(|i| (0..n).any(|j| s[j][*i] == '#'))
        .unwrap();

    let mut compress_s = vec![vec!['.'; max_w - min_w + 1]; max_h - min_h + 1];
    for i in min_h..=max_h {
        for j in min_w..=max_w {
            compress_s[i - min_h][j - min_w] = s[i][j];
        }
    }

    let min_h = (0..n).find(|i| (0..n).any(|j| t[*i][j] == '#')).unwrap();
    let max_h = (0..n)
        .rev()
        .find(|i| (0..n).any(|j| t[*i][j] == '#'))
        .unwrap();
    let min_w = (0..n).find(|i| (0..n).any(|j| t[j][*i] == '#')).unwrap();
    let max_w = (0..n)
        .rev()
        .find(|i| (0..n).any(|j| t[j][*i] == '#'))
        .unwrap();

    let mut compress_t = vec![vec!['.'; max_w - min_w + 1]; max_h - min_h + 1];
    for i in min_h..=max_h {
        for j in min_w..=max_w {
            compress_t[i - min_h][j - min_w] = t[i][j];
        }
    }

    for _ in 0..4 {
        if compress_s.len() == compress_t.len() && compress_s[0].len() == compress_t[0].len() {
            let mut found = true;
            for i in 0..compress_s.len() {
                for j in 0..compress_s[0].len() {
                    if compress_s[i][j] != compress_t[i][j] {
                        found = false;
                        break;
                    }
                }
            }

            if found {
                println!("Yes");
                return;
            }
        }

        let mut new_compress_t = vec![vec!['.'; compress_t.len()]; compress_t[0].len()];
        for i in 0..compress_t[0].len() {
            for j in 0..compress_t.len() {
                new_compress_t[i][j] = compress_t[compress_t.len() - 1 - j][i];
            }
        }

        compress_t = new_compress_t;
    }
    println!("No");
}
