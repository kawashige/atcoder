use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut c: Chars
    }

    let mut i = 0;
    let mut j = n - 1;
    let mut count = 0;
    while i < j {
        while i < n && c[i] == 'R' {
            i += 1;
        }

        if i == n {
            break;
        }

        while 0 < j && c[j] == 'W' {
            j -= 1;
        }

        if i < j {
            c.swap(i, j);
            i += 1;
            j -= 1;
            count += 1;
        }
    }

    println!("{}", count);
}
