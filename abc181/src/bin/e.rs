use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i32; n],
        mut w: [i32; m],
    }

    h.sort_unstable();
    w.sort_unstable();

    let mut sum = 0;
    for i in (1..n).step_by(2) {
        sum += h[i + 1] - h[i];
    }
    let mut j = 0;
    while j + 1 < w.len() && (h[0] - w[j + 1]).abs() <= (h[0] - w[j]).abs() {
        j += 1;
    }
    sum += (h[0] - w[j]).abs();

    let mut min = sum;
    for i in 1..n {
        sum -= (h[i - 1] - w[j]).abs();
        while j + 1 < w.len() && (h[i] - w[j + 1]).abs() <= (h[i] - w[j]).abs() {
            j += 1;
        }
        sum += (h[i] - w[j]).abs();
        sum += if i % 2 == 1 {
            h[i] - h[i - 1]
        } else {
            h[i - 1] - h[i]
        };

        min = std::cmp::min(min, sum);
    }

    println!("{}", min);
}
