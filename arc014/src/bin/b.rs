use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n]
    }

    let mut set = HashSet::new();
    set.insert(&w[0]);
    for i in 1..n {
        if set.contains(&w[i]) || w[i - 1].as_bytes().iter().last().unwrap() != &w[i].as_bytes()[0]
        {
            if i % 2 == 1 {
                println!("WIN");
            } else {
                println!("LOSE");
            }
            return;
        }
        set.insert(&w[i]);
    }
    println!("DRAW");
}
