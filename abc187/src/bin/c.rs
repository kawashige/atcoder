use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        strings: [String; n]
    }

    let mut set: HashSet<String> = HashSet::new();

    for s in strings {
        if s.starts_with("!") {
            let str = s.trim_start_matches("!");
            if set.contains(str) {
                println!("{}", str);
                return;
            }
        } else {
            if set.contains(&format!("!{}", s)) {
                println!("{}", s);
                return;
            }
        }
        set.insert(s);
    }
    println!("satisfiable");
}
