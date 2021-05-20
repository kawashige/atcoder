use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        xv: [(i64, i64); n]
    }

    let mut clockwise_acc = vec![0; n];
    let mut clockwise_max = vec![0; n];
    clockwise_acc[0] = xv[0].1 - xv[0].0;
    clockwise_max[0] = clockwise_acc[0];

    for i in 1..n {
        clockwise_acc[i] = clockwise_acc[i - 1] + xv[i].1 - (xv[i].0 - xv[i - 1].0);
        clockwise_max[i] = std::cmp::max(clockwise_max[i - 1], clockwise_acc[i]);
    }

    let mut counter_clockwise_acc = vec![0; n];
    let mut counter_clockwise_max = vec![0; n];
    counter_clockwise_acc[n - 1] = xv[n - 1].1 - (c - xv[n - 1].0);
    counter_clockwise_max[n - 1] = counter_clockwise_acc[n - 1];

    for i in (0..(n - 1)).rev() {
        counter_clockwise_acc[i] = counter_clockwise_acc[i + 1] + xv[i].1 - (xv[i + 1].0 - xv[i].0);
        counter_clockwise_max[i] =
            std::cmp::max(counter_clockwise_max[i + 1], counter_clockwise_acc[i]);
    }

    let mut max = 0;

    for i in 0..n {
        let p = clockwise_acc[i]
            + if i == n - 1 {
                0
            } else {
                (counter_clockwise_max[i + 1] - xv[i].0).max(0)
            };
        max = std::cmp::max(max, p);
    }

    for i in (0..n).rev() {
        let p = counter_clockwise_acc[i]
            + if i == 0 {
                0
            } else {
                (clockwise_max[i - 1] - (c - xv[i].0)).max(0)
            };
        max = std::cmp::max(max, p);
    }

    println!("{}", max);
}
