use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }

    let mut x = x.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    x.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut r = (0..n)
        .map(|i| {
            (
                x[i].0,
                if i >= n / 2 {
                    x[n / 2 - 1].1
                } else {
                    x[n / 2].1
                },
            )
        })
        .collect::<Vec<(usize, usize)>>();
    r.sort_unstable();

    for (_, result) in r {
        println!("{}", result);
    }
}
