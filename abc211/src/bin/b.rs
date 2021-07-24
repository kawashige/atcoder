use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: [String; 4]
    }

    if s.into_iter().collect::<HashSet<String>>().len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
