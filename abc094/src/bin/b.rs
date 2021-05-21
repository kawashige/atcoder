use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        a: [usize; m]
    }

    let (sum1, sum2) = a.into_iter().fold((0, 0), |(sum1, sum2), i| {
        if i < x {
            (sum1 + 1, sum2)
        } else {
            (sum1, sum2 + 1)
        }
    });

    println!("{}", sum1.min(sum2));
}
