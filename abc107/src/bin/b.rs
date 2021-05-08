use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut a: [Chars; h]
    }

    loop {
        let r = a.len();
        let c = a[0].len();
        a = a
            .into_iter()
            .filter(|row| row.iter().any(|c| c != &'.'))
            .collect();
        a = (0..a[0].len()).fold(vec![vec![]; a.len()], |mut v, i| {
            if (0..a.len()).any(|j| a[j][i] != '.') {
                (0..a.len()).for_each(|j| v[j].push(a[j][i]));
            }
            v
        });
        if a.len() == r && a[0].len() == c {
            break;
        }
    }

    for r in a {
        println!("{}", r.iter().collect::<String>());
    }
}
