use proconio::input;

fn main() {
    input! {
        _k: usize,
        t: usize,
        a: [usize; t]
    }

    let max = *a.iter().max().unwrap();
    let sum = a.iter().sum::<usize>();
    let r = if sum - max < max {
        max - (sum - max) - 1
    } else {
        0
    };

    println!("{}", r);
}
