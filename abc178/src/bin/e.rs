use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n]
    }

    let mut z_min = std::i64::MAX;
    let mut z_max = std::i64::MIN;
    let mut w_min = std::i64::MAX;
    let mut w_max = std::i64::MIN;

    for (x, y) in points {
        let z = x + y;
        let w = x - y;

        z_min = std::cmp::min(z_min, z);
        z_max = std::cmp::max(z_max, z);
        w_min = std::cmp::min(w_min, w);
        w_max = std::cmp::max(w_max, w);
    }

    println!("{}", std::cmp::max(z_max - z_min, w_max - w_min));
}
