use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    }

    let d1 = (1 - a).max(1 - b)..=(n - a).min(n - b);
    let d2 = (1 - a).max(b - n)..=(n - a).min(b - 1);

    for i in p..=q {
        println!(
            "{}",
            (r..=s)
                .map(|j| {
                    if (i - a == j - b && d1.contains(&(i - a)))
                        || (i - a == b - j && d2.contains(&(i - a)))
                    {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
}
