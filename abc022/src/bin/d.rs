use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(f64, f64); n],
        b: [(f64, f64); n]
    }

    let n = n as f64;

    let g_a = a
        .iter()
        .fold((0.0, 0.0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));

    let g_b = b
        .iter()
        .fold((0.0, 0.0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));

    let mut d_a = a
        .iter()
        .map(|(x, y)| {
            ((n * x - g_a.0) * (n * x - g_a.0) + (n * y - g_a.1) * (n * y - g_a.1)).sqrt()
        })
        .collect::<Vec<_>>();
    let mut d_b = b
        .iter()
        .map(|(x, y)| {
            ((n * x - g_b.0) * (n * x - g_b.0) + (n * y - g_b.1) * (n * y - g_b.1)).sqrt()
        })
        .collect::<Vec<_>>();

    d_a.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
    d_b.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());

    println!("{}", d_b.last().unwrap() / d_a.last().unwrap());
}
