use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        query: [usize; q]
    }

    let mut x_plus_y = xy.iter().map(|(x, y)| x + y).collect::<Vec<i64>>();
    let mut x_minus_y = xy.iter().map(|(x, y)| x - y).collect::<Vec<i64>>();
    x_plus_y.sort_unstable();
    x_minus_y.sort_unstable();

    for q in query {
        let q = q - 1;
        let r = ((xy[q].0 + xy[q].1) - x_plus_y[0])
            .max(x_plus_y.last().unwrap() - (xy[q].0 + xy[q].1))
            .max((xy[q].0 - xy[q].1) - x_minus_y[0])
            .max(x_minus_y.last().unwrap() - (xy[q].0 - xy[q].1));

        println!("{}", r);
    }
}
