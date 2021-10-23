use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let mut t = vec![0; n];

    for i in 0..m {
        t[a[i] - 1] = i + 1;
    }

    let mut t = t.into_iter().zip(1..).collect::<Vec<_>>();
    t.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    for (_, x) in t {
        println!("{}", x);
    }
}
