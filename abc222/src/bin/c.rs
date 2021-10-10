use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n]
    }

    let mut p = (0..2 * n).map(|i| (0, i)).collect::<Vec<_>>();

    for i in 0..m {
        let mut new_p = Vec::new();
        for j in 0..n {
            let j = 2 * j;
            if a[p[j].1][i] == a[p[j + 1].1][i] {
                new_p.push(p[j]);
                new_p.push(p[j + 1]);
            } else if (a[p[j].1][i] == 'G' && a[p[j + 1].1][i] == 'C')
                || (a[p[j].1][i] == 'C' && a[p[j + 1].1][i] == 'P')
                || (a[p[j].1][i] == 'P' && a[p[j + 1].1][i] == 'G')
            {
                new_p.push((p[j].0 + 1, p[j].1));
                new_p.push(p[j + 1]);
            } else {
                new_p.push(p[j]);
                new_p.push((p[j + 1].0 + 1, p[j + 1].1));
            }
        }
        new_p.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        p = new_p;
    }

    for (_, x) in p {
        println!("{}", x + 1);
    }
}
