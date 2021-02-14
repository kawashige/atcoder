use proconio::input;

fn main() {
    input! {
        n: usize,
        times: [usize; n]
    }

    let mut min = std::usize::MAX;
    for i in 0..2_usize.pow(5) {
        let mut time = [0, 0];
        for j in 0..times.len() {
            if i & 1 << j == 0 {
                time[0] += times[j];
            } else {
                time[1] += times[j];
            }
        }
        min = std::cmp::min(min, std::cmp::max(time[0], time[1]));
    }
    println!("{}", min);
}
