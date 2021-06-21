use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        ab: [(usize, i64); n]
    }

    let mut count = vec![0; 100_001];
    for (a, b) in ab {
        count[a] += b;
    }

    for i in 1..count.len() {
        k -= count[i];
        if k <= 0 {
            println!("{}", i);
            return;
        }
    }
}
