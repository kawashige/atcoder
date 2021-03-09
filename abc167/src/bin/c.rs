use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        books: [[usize; m + 1]; n]
    }

    let mut min = std::usize::MAX;
    let mut found = false;

    for i in 1..2_usize.pow(n as u32) {
        let mut sum = vec![0; m + 1];
        for j in 0..n {
            if i & 1 << j > 0 {
                for k in 0..=m {
                    sum[k] += books[j][k];
                }
            }
        }
        if sum[1..].iter().all(|s| s >= &x) {
            found = true;
            min = std::cmp::min(min, sum[0]);
        }
    }

    println!("{}", if found { min as i32 } else { -1 });
}
