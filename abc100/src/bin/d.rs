use proconio::input;

fn get_sum(v: &Vec<(i64, i64, i64)>, m: usize) -> i64 {
    let sum = v
        .iter()
        .take(m)
        .fold((0, 0, 0), |sum, v| (sum.0 + v.0, sum.1 + v.1, sum.2 + v.2));
    sum.0.abs() + sum.1.abs() + sum.2.abs()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xyz: [(i64, i64, i64); n]
    }

    let mut r = 0;

    xyz.sort_unstable_by_key(|v| v.0 + v.1 + v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| -v.0 - v.1 - v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| -v.0 + v.1 + v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| v.0 - v.1 + v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| v.0 + v.1 - v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| -v.0 - v.1 + v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| -v.0 + v.1 - v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    xyz.sort_unstable_by_key(|v| v.0 - v.1 - v.2);
    r = std::cmp::max(r, get_sum(&xyz, m));

    println!("{}", r);
}
