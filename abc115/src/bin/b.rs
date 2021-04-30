use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut sum = 0;
    let mut max = 0;
    for x in p {
        sum += x;
        max = std::cmp::max(max, x);
    }

    println!("{}", sum - max / 2);
}
