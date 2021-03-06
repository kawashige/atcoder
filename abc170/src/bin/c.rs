use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        x: i32,
        n: usize,
        p: [i32; n]
    }

    let set = p.into_iter().collect::<HashSet<i32>>();
    let mut i = 0;
    loop {
        if !set.contains(&(x - i)) {
            println!("{}", x - i);
            return;
        }
        if !set.contains(&(x + i)) {
            println!("{}", x + i);
            return;
        }
        i += 1;
    }
}
