use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    for i in 0..h {
        for i2 in i..h {
            for j in 0..w {
                for j2 in j..w {
                    if a[i][j] + a[i2][j2] > a[i2][j] + a[i][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
