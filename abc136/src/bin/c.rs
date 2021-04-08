use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    }

    for i in (0..(n - 1)).rev() {
        if h[i] > h[i + 1] {
            h[i] -= 1;
        }
    }

    if (0..(n - 1)).all(|i| h[i] <= h[i + 1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
