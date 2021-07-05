use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n]
    }

    let mut max = a[0];
    let mut min = a[0];
    let mut max_count = 1;
    let mut min_count = 1;

    let mut profits = Vec::new();

    for i in 1..n {
        if min == a[i] {
            min_count += 1;
        } else if max == a[i] {
            max_count += 1;
        } else if min > a[i] {
            if min < max {
                profits.push((max - min, max_count.min(min_count).min(t / 2)));
            }
            min = a[i];
            min_count = 1;
            max = a[i];
            max_count = 1;
        } else if max < a[i] {
            max = a[i];
            max_count = 1;
        }
    }
    if min < max {
        profits.push((max - min, max_count.min(min_count).min(t / 2)));
    }

    let mut r = 0;
    if !profits.is_empty() {
        profits.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let target = profits[0].0;
        for i in 0..profits.len() {
            if profits[i].0 != target {
                break;
            }
            r += profits[i].1;
        }
    }

    println!("{}", r);
}
