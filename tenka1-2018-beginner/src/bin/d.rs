use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut map = HashMap::new();
    let mut k = 2;
    let mut x = 1;
    while x <= n {
        map.insert(x, k);
        k += 1;
        x = k * (k - 1) / 2;
    }

    if let Some(k) = map.get(&n) {
        let mut r = vec![vec![]; *k];
        let mut i = 0;
        let mut j = i + 1;
        for x in 1..=n {
            r[i].push(x);
            r[j].push(x);
            j += 1;
            if r[i].len() == k - 1 {
                i += 1;
                j = i + 1;
            }
        }

        println!("Yes");
        println!("{}", r.len());
        for v in r {
            println!(
                "{} {}",
                v.len(),
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    } else {
        println!("No")
    }
}
