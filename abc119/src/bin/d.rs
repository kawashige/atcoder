use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        s: [i64; a],
        t: [i64; b],
        x: [i64; q],
    }

    let m: i64 = 100_000_000_000;

    let mut query = x.into_iter().enumerate().collect::<Vec<(usize, i64)>>();
    query.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let s = std::iter::once(-m)
        .chain(s.into_iter())
        .chain(std::iter::once(m))
        .collect::<Vec<i64>>();
    let t = std::iter::once(-m)
        .chain(t.into_iter())
        .chain(std::iter::once(m))
        .collect::<Vec<i64>>();

    let mut s_i = 1;
    let mut t_i = 1;

    let mut results = Vec::with_capacity(q);

    for (i, x) in query {
        while x > s[s_i] {
            s_i += 1;
        }
        while x > t[t_i] {
            t_i += 1;
        }

        let (s_left, s_right) = if s[s_i] == x {
            (s[s_i], s[s_i])
        } else {
            (s[s_i - 1], s[s_i])
        };

        let (t_left, t_right) = if t[t_i] == x {
            (t[t_i], t[t_i])
        } else {
            (t[t_i - 1], t[t_i])
        };

        let mut min = std::i64::MAX;

        min = std::cmp::min(min, std::cmp::max(s_right, t_right) - x);
        min = std::cmp::min(min, x - std::cmp::min(s_left, t_left));
        min = std::cmp::min(
            min,
            std::cmp::min(x - s_left, t_right - x) + t_right - s_left,
        );
        min = std::cmp::min(
            min,
            std::cmp::min(x - t_left, s_right - x) + s_right - t_left,
        );

        results.push((i, min));
    }

    results.sort_unstable();
    for (_, r) in results {
        println!("{}", r);
    }
}
