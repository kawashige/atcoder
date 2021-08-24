use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        s: Chars
    }

    let mut chars = s.iter().fold([0; 26], |mut chars, c| {
        chars[*c as usize - b'a' as usize] += 1;
        chars
    });

    let mut r = String::new();

    for i in 0..n {
        for j in 0..26 {
            if chars[j] > 0 {
                if s[i] == (b'a' + j as u8) as char {
                    chars[j] -= 1;
                    r.push((b'a' + j as u8) as char);
                    break;
                }

                if k == 0 {
                    continue;
                }

                chars[j] -= 1;
                k -= 1;
                if k >= s[(i + 1)..]
                    .iter()
                    .fold([0; 26], |mut chars, c| {
                        chars[*c as usize - b'a' as usize] += 1;
                        chars
                    })
                    .iter()
                    .zip(chars.iter())
                    .map(|(c1, c2)| (*c2 as i32 - *c1 as i32).max(0) as usize)
                    .sum::<usize>()
                {
                    r.push((b'a' + j as u8) as char);
                    break;
                }
                k += 1;
                chars[j] += 1;
            }
        }
    }

    println!("{}", r);
}
