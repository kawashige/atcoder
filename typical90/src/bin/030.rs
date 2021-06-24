use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut count = vec![0; n + 1];
    let mut r = 0;
    for i in 2..count.len() {
        if count[i] > 0 {
            continue;
        }
        for j in (i..count.len()).step_by(i) {
            count[j] += 1;
        }
    }

    for i in 2..count.len() {
        if count[i] >= k {
            r += 1;
        }
    }

    println!("{}", r);
}
