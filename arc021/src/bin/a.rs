use proconio::input;

fn main() {
    input! {
        a: [[usize; 4]; 4]
    }

    for i in 0..4 {
        for j in 0..4 {
            if (i > 0 && a[i][j] == a[i - 1][j])
                || (j > 0 && a[i][j] == a[i][j - 1])
                || (i < 3 && a[i][j] == a[i + 1][j])
                || (j < 3 && a[i][j] == a[i][j + 1])
            {
                println!("CONTINUE");
                return;
            }
        }
    }
    println!("GAMEOVER");
}
