use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [[String; w]; h]
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == "snuke" {
                println!("{}{}", (j as u8 + b'A') as char, i + 1);
                return;
            }
        }
    }
}
