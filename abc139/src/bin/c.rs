use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut max = 0;
    let mut tmp = 0;
    for i in 1..n {
        if h[i - 1] >= h[i] {
            tmp += 1;
            max = std::cmp::max(tmp, max);
        } else {
            tmp = 0;
        }
    }

    println!("{}", max);
}
