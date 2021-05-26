use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    for i in 0..n {
        let mut angles = Vec::with_capacity(n - 1);
        for j in 0..n {
            if i == j {
                continue;
            }
            angles.push(
                ((xy[j].1 - xy[i].1) as f64 / (xy[j].0 - xy[i].0) as f64).atan() * 180.0
                    / std::f64::consts::PI,
            );
        }
        angles.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
        let mut i = 0;
        let mut j = 1;
        let mut max = 0;
        while j < angles.len() {
            while angles[j] - angles[i] > 180.0 {
                i += 1;
            }
        }
    }
}
