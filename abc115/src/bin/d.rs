use proconio::input;

fn recurse(counts: &Vec<u64>, p_counts: &Vec<u64>, x: u64, n: usize) -> u64 {
    if x == 0 {
        0
    } else if x == counts[n] {
        p_counts[n]
    } else if x < counts[n - 1] + 2 {
        recurse(counts, p_counts, x - 1, n - 1)
    } else {
        p_counts[n - 1]
            + 1
            + recurse(
                counts,
                p_counts,
                std::cmp::min(x - counts[n - 1] - 2, counts[n - 1]),
                n - 1,
            )
    }
}

fn main() {
    input! {
        n: usize,
        x: u64
    }

    let mut counts = vec![0_u64; n + 1];
    let mut p_counts = vec![0_u64; n + 1];
    counts[0] = 1;
    p_counts[0] = 1;
    for i in 1..=n {
        counts[i] = 3 + counts[i - 1] * 2;
        p_counts[i] = 1 + p_counts[i - 1] * 2;
    }

    let r = recurse(&counts, &p_counts, x, n);

    println!("{}", r);
}
