use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut mins = Vec::with_capacity(n);
    let mut maxs = Vec::with_capacity(n);
    for (a, b) in ab {
        mins.push(a);
        maxs.push(b);
    }
    mins.sort_unstable();
    maxs.sort_unstable();

    let median = if n % 2 == 0 {
        maxs[n / 2] + maxs[n / 2 - 1] - mins[n / 2] - mins[n / 2 - 1] + 1
    } else {
        maxs[(n + 1) / 2 - 1] - mins[(n + 1) / 2 - 1] + 1
    };

    println!("{}", median);
}
