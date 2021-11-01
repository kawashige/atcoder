use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n]
    }

    if 7 < (b[0][0] - 1) % 7 + m {
        println!("No");
        return;
    }

    for i in 0..n {
        for j in 0..m {
            if b[i][j] - b[0][0] != i * 7 + j {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
