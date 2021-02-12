use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let a = a % b;
    for n in 0..=b {
        if (a * n) % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
