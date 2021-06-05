use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: [Chars; 2]
    }

    if c[0][0] == c[1][2] && c[0][1] == c[1][1] && c[0][2] == c[1][0] {
        println!("YES");
    } else {
        println!("NO");
    }
}
