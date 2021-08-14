use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize
    }

    let mut r = 0;
    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    r += 1;
                }
            }
        }
    }

    println!("{}", r);
}
