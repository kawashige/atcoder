use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let mut r = 0;
    for a in 1..=k {
        for b in 1..=k {
            let ab = a * b;
            if ab > k {
                break;
            }
            for c in 1..=k {
                if ab * c > k {
                    break;
                }
                r += 1;
            }
        }
    }

    println!("{}", r);
}
