use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h]
    }

    let mut min = std::usize::MAX;
    for i in 0..2_usize.pow(h as u32) {
        let mut count = i.count_ones() as usize;
        let mut counts = vec![0; i.count_ones() as usize + 1];

        for j in 0..w {
            let mut tmp = vec![0; i.count_ones() as usize + 1];
            let mut l = 0;
            for m in 0..h {
                if s[m][j] == '1' {
                    tmp[l] += 1;
                }
                if i & 1 << m > 0 {
                    l += 1;
                }
            }

            if (0..counts.len()).all(|m| counts[m] + tmp[m] <= k) {
                (0..counts.len()).for_each(|m| counts[m] += tmp[m]);
            } else if (0..tmp.len()).all(|m| tmp[m] <= k) {
                count += 1;
                counts = tmp;
            } else {
                count = std::usize::MAX;
                break;
            }
        }

        min = std::cmp::min(min, count);
    }

    println!("{}", min);
}
