use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }

    let mut score = vec![false; 100 * n + 1];
    score[0] = true;
    for i in 0..n {
        for j in (0..score.len()).rev() {
            if score[j] {
                score[j + s[i]] = true;
            }
        }
    }

    let i = (0..score.len())
        .rev()
        .find(|i| i % 10 != 0 && score[*i])
        .unwrap_or(0);

    println!("{}", i);
}
