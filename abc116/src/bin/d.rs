use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut td: [(usize, usize); n],
    }

    td.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut duplicate = Vec::with_capacity(k);
    let mut count = vec![0; n + 1];
    let mut x = 0;
    let mut sum = 0;

    for i in 0..k {
        if count[td[i].0] == 0 {
            x += 1;
        } else {
            duplicate.push(td[i]);
        }
        count[td[i].0] += 1;
        sum += td[i].1;
    }

    let mut r = sum as u64 + (x * x) as u64;

    for i in k..n {
        if x == k {
            break;
        }

        if count[td[i].0] == 0 {
            count[td[i].0] += 1;
            sum += td[i].1;
            x += 1;
            let (_, d) = duplicate.pop().unwrap();
            sum -= d;
            r = std::cmp::max(r, sum as u64 + (x * x) as u64);
        }
    }

    println!("{}", r);
}
