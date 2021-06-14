use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let m = 998244353;
    let mut r = 1;

    for i in 0..h {
        let mut count = vec![0; 3];

        for j in 0..=std::cmp::min(i, w - 1) {
            match s[i - j][j] {
                'R' => count[0] += 1,
                'B' => count[1] += 1,
                _ => count[2] += 1,
            }
        }
        if count[0] > 0 && count[1] > 0 {
            println!("0");
            return;
        } else if count[0] == 0 && count[1] == 0 {
            r = r * 2 % m;
        }
    }
    for i in 1..w {
        let mut count = vec![0; 3];

        for j in 0..=std::cmp::min(h - 1, w - 1 - i) {
            match s[h - 1 - j][i + j] {
                'R' => count[0] += 1,
                'B' => count[1] += 1,
                _ => count[2] += 1,
            }
        }
        if count[0] > 0 && count[1] > 0 {
            println!("0");
            return;
        } else if count[0] == 0 && count[1] == 0 {
            r = r * 2 % m;
        }
    }

    println!("{}", r);
}
