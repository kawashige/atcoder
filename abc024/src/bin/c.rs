use proconio::input;

fn main() {
    input! {
        _n: usize,
        d: usize,
        k: usize,
        lr: [(usize, usize); d],
        st: [(usize, usize); k]
    }

    let mut result = vec![0; k];

    let mut st = st.into_iter().map(|(s, t)| ((s, s), t)).collect::<Vec<_>>();

    for (i, (l, r)) in lr.into_iter().enumerate() {
        for j in 0..k {
            if result[j] != 0 {
                continue;
            }

            if l.max(st[j].0 .0) <= r.min(st[j].0 .1) {
                st[j].0 .0 = l.min(st[j].0 .0);
                st[j].0 .1 = r.max(st[j].0 .1);
            }

            if (st[j].0 .0..=st[j].0 .1).contains(&st[j].1) {
                result[j] = i + 1;
            }
        }
    }

    for r in result {
        println!("{}", r);
    }
}
