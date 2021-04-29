use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: usize
    }

    let mut set = HashSet::new();
    set.insert(s);

    let mut count = 1;
    let mut n = s;
    loop {
        count += 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        if !set.insert(n) {
            println!("{}", count);
            return;
        }
    }
}
