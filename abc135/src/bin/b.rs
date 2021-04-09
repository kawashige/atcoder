use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n]
    }

    for i in 0..n {
        for j in 0..n {
            p.swap(i, j);
            if (1..n).all(|k| p[k - 1] < p[k]) {
                println!("YES");
                return;
            }
            p.swap(j, i);
        }
    }
    println!("NO");
}
