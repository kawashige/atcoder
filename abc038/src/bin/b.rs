use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        h2: usize,
        w2: usize,
    }

    if h1 == h2 || h1 == w2 || h2 == w1 || w1 == w2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
