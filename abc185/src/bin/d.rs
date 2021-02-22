use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    if m == 0 {
        println!("1");
        return;
    }

    let mut a: Vec<usize> = a;
    a.push(0);
    a.push(n + 1);
    a.sort_unstable();

    let mut min = std::usize::MAX;
    let mut white = vec![];
    for i in 1..a.len() {
        let d = a[i] - a[i - 1] - 1;
        if d > 0 {
            white.push(d);
            min = std::cmp::min(min, d);
        }
    }

    let count = white.iter().map(|w| (w + min - 1) / min).sum::<usize>();
    println!("{}", count);
}
