use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        prime_c: usize,
        services: [(usize, usize, usize); n],
    }

    let cost = services
        .into_iter()
        .fold(BTreeMap::new(), |mut map, (a, b, c)| {
            *map.entry(a).or_insert(0_i128) += c as i128;
            *map.entry(b + 1).or_insert(0_i128) -= c as i128;
            map
        })
        .into_iter()
        .fold((0, 0_u128, 0_u128), |(prev, cost, total_cost), (i, c)| {
            (
                i,
                (cost as i128 + c) as u128,
                total_cost
                    + if cost == 0 {
                        0
                    } else {
                        ((i - prev) * std::cmp::min(cost as usize, prime_c)) as u128
                    },
            )
        })
        .2;

    println!("{}", cost);
}
