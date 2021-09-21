use proconio::input;

fn main() {
    input! {
        n: usize,
        xl: [(i64, i64); n]
    }

    let mut robots = xl
        .into_iter()
        .map(|(x, l)| (x - l, x + l))
        .collect::<Vec<_>>();

    robots.sort_unstable_by_key(|(_, e)| *e);

    let mut last_e = std::i64::MIN;
    let mut count = 0;
    for (s, e) in robots {
        if last_e <= s {
            count += 1;
            last_e = e;
        }
    }

    println!("{}", count);
}
