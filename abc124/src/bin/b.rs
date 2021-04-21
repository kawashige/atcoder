use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut count = 1;
    let mut max = h[0];
    for i in 1..n {
        if max <= h[i] {
            count += 1;
        }
        max = std::cmp::max(max, h[i]);
    }

    println!("{}", count);
}
