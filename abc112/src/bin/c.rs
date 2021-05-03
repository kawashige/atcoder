use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xyh: [(i32, i32, i32); n]
    }

    xyh.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    for i in 0..101 {
        for j in 0..101 {
            let h = xyh[0].2 + (i - xyh[0].0).abs() + (j - xyh[0].1).abs();
            let mut valid = true;
            for (x, y, hh) in &xyh {
                if hh == &0 {
                    if h > (i - x).abs() + (j - y).abs() {
                        valid = false;
                        break;
                    }
                } else {
                    if h != hh + (i - x).abs() + (j - y).abs() {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                println!("{} {} {}", i, j, h);
                return;
            }
        }
    }
}
