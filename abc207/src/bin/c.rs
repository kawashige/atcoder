use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, u64, u64); n]
    }

    let mut lr = tlr
        .into_iter()
        .map(|(t, l, r)| match t {
            1 => (l * 2, r * 2),
            2 => (l * 2, r * 2 - 1),
            3 => (l * 2 + 1, r * 2),
            _ => (l * 2 + 1, r * 2 - 1),
        })
        .collect::<Vec<(u64, u64)>>();

    lr.sort_unstable();

    let mut r = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if lr[i].1 >= lr[j].0 {
                r += 1;
            }
        }
    }

    println!("{}", r);
}
