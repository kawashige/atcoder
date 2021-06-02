use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        stc: [(usize, usize, usize); n]
    }

    let mut times = vec![0; 100_001];
    for (s, t, _) in stc {
        times[s - 1] += 1;
        times[t] -= 1;
    }

    let mut max = 0;
    let mut acc = 0;
    for i in 0..times.len() {
        acc += times[i];
        max = std::cmp::max(max, acc);
    }

    println!("{}", std::cmp::min(max as usize, c));
}
