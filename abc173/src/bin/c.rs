use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [Chars; h]
    }

    let mut count = 0;
    for i in 0..2_usize.pow(h as u32) {
        for j in 0..2_usize.pow(w as u32) {
            let mut sum = 0;
            for k in 0..h {
                if i & 1 << k == 0 {
                    continue;
                }
                for l in 0..w {
                    if j & 1 << l == 0 {
                        continue;
                    }
                    if grid[k][l] == '#' {
                        sum += 1;
                    }
                }
            }
            if sum == k {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
