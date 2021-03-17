use std::{collections::VecDeque, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let s: String = read();
    let q: usize = read();
    let queries = (0..q).map(|_| {
        read::<String>()
            .replace(" ", "")
            .chars()
            .collect::<Vec<char>>()
    });

    let mut chars = s.chars().collect::<VecDeque<char>>();
    let mut reverse = false;

    for q in queries {
        if q[0] == '1' {
            reverse = !reverse;
        } else {
            match (q[1], reverse) {
                ('1', false) | ('2', true) => {
                    chars.push_front(q[2]);
                }
                ('1', true) | ('2', false) => {
                    chars.push_back(q[2]);
                }
                _ => unreachable!(),
            }
        }
    }

    let s: String = if reverse {
        chars.into_iter().rev().collect()
    } else {
        chars.into_iter().collect()
    };
    println!("{}", s);
}
