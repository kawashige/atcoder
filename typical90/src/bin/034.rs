use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut count = HashMap::new();
    let mut max = 0;
    let mut j = 0;
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
        while count.len() > k {
            let c = count.get_mut(&a[j]).unwrap();
            *c -= 1;
            if *c == 0 {
                count.remove(&a[j]);
            }
            j += 1;
        }
        max = std::cmp::max(max, i - j + 1);
    }

    println!("{}", max);
}
