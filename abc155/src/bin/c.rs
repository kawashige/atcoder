use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut count = HashMap::new();
    for ss in s {
        *count.entry(ss).or_insert(0) += 1;
    }

    let mut v = count.into_iter().collect::<Vec<(String, usize)>>();
    v.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let max_count = v[0].1;
    for (ss, count) in v {
        if count != max_count {
            break;
        }
        println!("{}", ss);
    }
}
