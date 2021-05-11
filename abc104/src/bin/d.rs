use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let m = 1_000_000_007;

    let mut left_a = vec![0; s.len()];
    let mut left_wildcard = vec![0; s.len()];
    let mut right_c = vec![0; s.len()];
    let mut right_wildcard = vec![0; s.len()];

    let mut a_count = 0;
    let mut wildcard_count = 0;
    for i in 0..s.len() {
        left_a[i] = a_count;
        left_wildcard[i] = wildcard_count;
        if s[i] == 'A' {
            a_count += 1;
        } else if s[i] == '?' {
            wildcard_count += 1;
        }
    }

    let mut c_count = 0;
    let mut wildcard_count = 0;
    for i in (0..s.len()).rev() {
        right_c[i] = c_count;
        right_wildcard[i] = wildcard_count;
        if s[i] == 'C' {
            c_count += 1;
        } else if s[i] == '?' {
            wildcard_count += 1;
        }
    }

    let mut pow = vec![1; s.len() + 1];
    for i in 1..=s.len() {
        pow[i] = pow[i - 1] * 3 % m;
    }

    let mut count = 0;
    for i in 1..s.len() {
        if s[i] == 'B' || s[i] == '?' {
            count += ((left_a[i] * pow[left_wildcard[i]] % m
                + pow[left_wildcard[i].saturating_sub(1)] * left_wildcard[i] % m)
                % m)
                * ((right_c[i] * pow[right_wildcard[i]] % m
                    + pow[right_wildcard[i].saturating_sub(1)] * right_wildcard[i] % m)
                    % m);
            count %= m
        }
    }

    println!("{}", count);
}
