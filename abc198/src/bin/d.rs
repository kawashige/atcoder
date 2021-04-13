use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};

fn dfs(
    chars: &Vec<char>,
    char_nums: &mut Vec<i64>,
    num_used: &mut [bool; 10],
    coefficients: &HashMap<char, i64>,
    not_zero: &[char; 3],
) -> bool {
    if char_nums.len() == chars.len() {
        return (0..chars.len()).fold(0, |acc, i| acc + char_nums[i] * coefficients[&chars[i]])
            == 0;
    }

    for i in 0..10 {
        if num_used[i]
            || (i == 0
                && (chars[char_nums.len()] == not_zero[0]
                    || chars[char_nums.len()] == not_zero[1]
                    || chars[char_nums.len()] == not_zero[2]))
        {
            continue;
        }
        num_used[i] = true;
        char_nums.push(i as i64);

        if dfs(chars, char_nums, num_used, coefficients, not_zero) {
            return true;
        }

        num_used[i] = false;
        char_nums.pop();
    }

    false
}

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

    let chars = s1
        .iter()
        .cloned()
        .chain(s2.iter().cloned())
        .chain(s3.iter().cloned())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();

    if chars.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let mut coefficients = HashMap::new();
    for (i, c) in s1.iter().rev().enumerate() {
        *coefficients.entry(*c).or_insert(0) += 10_i64.pow(i as u32)
    }
    for (i, c) in s2.iter().rev().enumerate() {
        *coefficients.entry(*c).or_insert(0) += 10_i64.pow(i as u32)
    }
    for (i, c) in s3.iter().rev().enumerate() {
        *coefficients.entry(*c).or_insert(0) -= 10_i64.pow(i as u32)
    }

    let not_zero = [s1[0], s2[0], s3[0]];

    let mut num_used = [false; 10];
    let mut char_nums = vec![];

    if dfs(
        &chars,
        &mut char_nums,
        &mut num_used,
        &coefficients,
        &not_zero,
    ) {
        let n1 = s1.into_iter().fold(0_i64, |acc, c| {
            10 * acc + char_nums[(0..chars.len()).find(|i| chars[*i] == c).unwrap()]
        });
        let n2 = s2.into_iter().fold(0_i64, |acc, c| {
            10 * acc + char_nums[(0..chars.len()).find(|i| chars[*i] == c).unwrap()]
        });
        let n3 = s3.into_iter().fold(0_i64, |acc, c| {
            10 * acc + char_nums[(0..chars.len()).find(|i| chars[*i] == c).unwrap()]
        });
        println!("{}", n1);
        println!("{}", n2);
        println!("{}", n3);
    } else {
        println!("UNSOLVABLE");
    }
}
