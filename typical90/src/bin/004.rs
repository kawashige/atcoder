use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    let mut sum_h = vec![0; h];
    let mut sum_w = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_h[i] += a[i][j];
            sum_w[j] += a[i][j];
        }
    }

    for i in 0..h {
        let s = (0..w)
            .map(|j| (sum_h[i] + sum_w[j] - a[i][j]).to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", s);
    }
}
