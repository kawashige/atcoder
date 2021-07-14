use proconio::input;
use proconio::marker::Chars;

fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let r = mod_pow(x, n / 2, m);
        r * r % m
    } else {
        x * mod_pow(x, n - 1, m) % m
    }
}

fn main() {
    input! {
        n: usize,
        x: Chars
    }

    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = dp[i % i.count_ones() as usize] + 1;
    }

    let ones = x.iter().filter(|c| c == &&'1').count();
    let plus = x.iter().fold(0, |acc, c| {
        (acc * 2 + if c == &'0' { 0 } else { 1 }) % (ones + 1)
    });
    let minus = if ones < 2 {
        0
    } else {
        x.iter().fold(0, |acc, c| {
            (acc * 2 + if c == &'0' { 0 } else { 1 }) % (ones - 1)
        })
    };

    for i in 0..x.len() {
        let num = if x[i] == '0' {
            (plus + mod_pow(2, x.len() - 1 - i, ones + 1)) % (ones + 1)
        } else {
            if ones == 1 {
                println!("0");
                continue;
            }
            ((ones - 1) + minus - mod_pow(2, x.len() - 1 - i, ones - 1)) % (ones - 1)
        };

        println!("{}", dp[num] + 1);
    }
}
