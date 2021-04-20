use proconio::input;
use std::collections::BTreeMap;

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let target = a
        .iter()
        .fold(BTreeMap::new(), |mut map, i| {
            *map.entry(*i).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter(|(_, v)| v == &1)
        .take(2)
        .map(|(k, _)| k)
        .collect::<Vec<usize>>();

    if target.is_empty() {
        let result = (1..n).fold(a[0], |r, j| gcd(r, a[j]));
        println!("{}", result);
    } else if target.len() == 1 {
        let r = if a[0] == target[0] { a[1] } else { a[0] };
        let result = (0..n).fold(r, |result, j| {
            if a[j] == target[0] {
                result
            } else {
                gcd(result, a[j])
            }
        });
        println!("{}", result);
    } else {
        let mut gcd1 = target[0];
        let mut gcd2 = target[1];
        for i in 0..n {
            if a[i] != target[1] {
                gcd1 = gcd(gcd1, a[i]);
            }
            if a[i] != target[0] {
                gcd2 = gcd(gcd2, a[i]);
            }
        }

        println!("{}", std::cmp::max(gcd1, gcd2));
    }
}
