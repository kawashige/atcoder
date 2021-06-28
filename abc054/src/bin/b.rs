use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
        b: [Chars; m]
    }

    if n < m {
        println!("No");
        return;
    }

    for i in 0..(n - m + 1) {
        for j in 0..(n - m + 1) {
            if (0..m).all(|k| b[k].as_slice() == &a[i + k][j..(j + m)]) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
