use proconio::input;

fn main() {
    input! {
        n: usize,
        at: [(i64, usize); n],
        q: usize,
        x: [i64; q]
    }

    let mut min = None;
    let mut max = None;
    let mut num: i64 = 0;

    for (a, t) in at {
        match t {
            1 => {
                num += a;
                min = min.map(|i| i + a);
                max = max.map(|i| i + a);
            }
            2 => {
                max = max.map(|i| std::cmp::max(i, a)).or(Some(a));
                min = min.map(|i| std::cmp::max(i, a));
            }
            3 => {
                min = min.map(|i| std::cmp::min(i, a)).or(Some(a));
                max = max.map(|i| std::cmp::min(i, a));
            }
            _ => unreachable!(),
        }
    }

    for xx in x {
        let r = match (min, max) {
            (None, None) => xx + num,
            (None, Some(i)) => std::cmp::max(i, xx + num),
            (Some(i), None) => std::cmp::min(i, xx + num),
            (Some(i), Some(j)) => std::cmp::max(j, std::cmp::min(i, xx + num)),
        };
        println!("{}", r);
    }
}
