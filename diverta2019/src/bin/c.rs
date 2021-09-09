use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut r = 0;

    let mut end_a = 0;
    let mut start_b = 0;
    let mut end_a_start_b = 0;

    for x in s {
        if x[0] == 'B' {
            start_b += 1;
        }
        if x.last() == Some(&'A') {
            end_a += 1;
        }
        if x[0] == 'B' && x.last() == Some(&'A') {
            end_a_start_b += 1;
        }

        r += (0..(x.len() - 1))
            .filter(|i| x[*i] == 'A' && x[i + 1] == 'B')
            .count();
    }

    if end_a > 0 && end_a == start_b && end_a == end_a_start_b {
        r += end_a - 1;
    } else {
        r += end_a.min(start_b);
    }

    println!("{}", r);
}
