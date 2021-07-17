use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n]
    }

    let mut count = HashMap::new();
    for i in 0..k {
        *count.entry(c[i]).or_insert(0) += 1;
    }

    let mut r = count.keys().len();

    for i in k..n {
        *count.entry(c[i]).or_insert(0) += 1;
        if count.get(&c[i - k]).unwrap() == &1 {
            count.remove(&c[i - k]);
        } else {
            *count.get_mut(&c[i - k]).unwrap() -= 1;
        }
        r = r.max(count.keys().len());
    }

    println!("{}", r);
}
