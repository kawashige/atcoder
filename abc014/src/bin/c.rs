use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut count = vec![0; 1000001];

    for (a, b) in ab {
        count[a] += 1;
        if b < count.len() - 1 {
            count[b + 1] -= 1;
        }
    }

    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    println!("{}", count.into_iter().max().unwrap());
}
