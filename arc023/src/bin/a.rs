use proconio::input;

fn days(y: usize, m: usize, d: usize) -> usize {
    let mm = m + if m == 1 || m == 2 { 12 } else { 0 };
    let yy = y - if m == 1 || m == 2 { 1 } else { 0 };
    365 * yy + yy / 4 - yy / 100 + yy / 400 + 306 * (mm + 1) / 10 + d - 429
}

fn main() {
    input! {
        y: usize,
        m: usize,
        d: usize
    }

    println!("{}", days(2014, 5, 17) - days(y, m, d));
}
