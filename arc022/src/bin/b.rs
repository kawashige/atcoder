use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut set = HashSet::new();
    set.insert(a[0]);

    let mut i = 0;
    let mut r = 1;
    for j in 1..n {
        if set.contains(&a[j]) {
            while a[i] != a[j] {
                set.remove(&a[i]);
                i += 1;
            }
            i += 1;
        } else {
            r = r.max(j - i + 1);
        }
        set.insert(a[j]);
    }

    println!("{}", r);
}
