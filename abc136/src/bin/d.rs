use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut counts = vec![0; s.len()];

    let mut i = 0;
    while i < s.len() {
        let mut count = 1;
        while s[i + 1] == 'R' {
            count += 1;
            i += 1;
        }

        counts[i + 1] = count / 2;
        counts[i] = count - count / 2;

        i += 1;
        while i < s.len() && s[i] == 'L' {
            i += 1;
        }
    }

    let mut i = 0;
    while i < s.len() {
        let mut count = 1;
        while s[s.len() - 1 - i - 1] == 'L' {
            count += 1;
            i += 1;
        }

        counts[s.len() - 1 - i - 1] += count / 2;
        counts[s.len() - 1 - i] += count - count / 2;

        i += 1;
        while i < s.len() && s[s.len() - 1 - i] == 'R' {
            i += 1;
        }
    }

    println!(
        "{}",
        counts
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
