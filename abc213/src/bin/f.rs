use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    for x in 0..n {
        let mut z = vec![0; n - x];
        z[0] = n - x;
        let mut i = x + 1;
        let mut j = x;

        while i < s.len() {
            while i + j < s.len() && s[j] == s[i + j] {
                j += 1;
            }
            z[i] = j;
            if j == x {
                i += 1;
                continue;
            }

            let mut k = 1;
            while k < j && k + z[k] < j {
                z[i + k] = z[k];
                k += 1;
            }
            i += k;
            j -= k;
        }

        println!("z: {:?}", z);
    }
}
