use proconio::input;
use proconio::marker::Chars;

fn recurse(chars: Vec<[usize; 26]>, k: usize, b: usize) {
    if k == 0 {
        if chars.len() == 0 {
            println!("{}", b);
        } else if chars.len() == 1 {
            println!("impossible");
        } else {
            let mut min = std::usize::MAX;
            let count = chars[0].iter().sum::<usize>();
            for i in 0..(chars.len() / 2) {
                for j in 0..26 {
                    let tmp = count - chars[i][j];
                    for k in 0..26 {
                        if j == k {
                            continue;
                        }
                        min = std::cmp::min(min, tmp + count - chars[chars.len() - 1 - i][k]);
                    }
                }
            }
            let mut new_b = 0;
            if chars.len() % 2 == 1 {
                new_b += chars[chars.len() / 2].iter().sum::<usize>()
                    - chars[chars.len() / 2].iter().max().unwrap();
            }
            println!("{}", min + b + new_b);
        }
    } else if chars.len() == 0 {
        println!("impossible");
    } else {
        let mut v = vec![[0; 26]; chars.len() / 2];
        for i in 0..(chars.len() / 2) {
            for j in 0..26 {
                v[i][j] += chars[i][j] + chars[chars.len() - 1 - i][j];
            }
        }
        let mut new_b = 0;
        if chars.len() % 2 == 1 {
            new_b += chars[chars.len() / 2].iter().sum::<usize>()
                - chars[chars.len() / 2].iter().max().unwrap();
        }
        recurse(v, k - 1, new_b + b)
    }
}

fn main() {
    input! {
        k: usize,
        s: Chars
    }

    let mut v = vec![[0; 26]; s.len()];
    for i in 0..s.len() {
        v[i][s[i] as usize - 0x61] = 1;
    }

    recurse(v, k, 0);
}
