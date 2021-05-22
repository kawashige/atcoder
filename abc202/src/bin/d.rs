use std::collections::HashMap;

use proconio::input;

pub fn count(a: u64, b: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if a == 0 || b == 0 {
        1
    } else {
        if let Some(n) = memo.get(&(a, b)) {
            *n
        } else {
            let n = count(a - 1, b, memo) + count(a, b - 1, memo);
            memo.insert((a, b), n);
            n
        }
    }
}

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut k: u64
    }

    let mut s = String::new();
    let mut memo = HashMap::new();
    for _ in 0..(a + b) {
        let c = count(a - 1, b, &mut memo);
        if k > c {
            k -= c;
            s.push('b');
            b -= 1;
        } else {
            s.push('a');
            a -= 1;
        }

        if a == 0 {
            for _ in 0..b {
                s.push('b');
            }
            break;
        }

        if b == 0 {
            for _ in 0..a {
                s.push('a');
            }
            break;
        }
    }

    println!("{}", s);
}
