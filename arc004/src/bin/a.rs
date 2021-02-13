use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i32, i32); n]
    }

    let mut max = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = (points[i].0 - points[j].0).pow(2) + (points[i].1 - points[j].1).pow(2);
            max = std::cmp::max(max, d);
        }
    }
    println!("{}", (max as f64).sqrt());
}
