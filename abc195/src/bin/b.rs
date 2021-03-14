use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize
    }

    let mut count = 1;
    let mut min = std::usize::MAX;
    let mut max = 0;
    while count <= 1000 * w {
        let num = 1000.0 * w as f64 / count as f64;
        if a as f64 <= num && num <= b as f64 {
            min = std::cmp::min(min, count);
            max = std::cmp::max(max, count);
        }
        count += 1;
    }

    if min == std::usize::MAX || max == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}
