use proconio::input;

fn main() {
    input! {
        k: i32,
        s: i32
    }

    let mut count = 0;
    for x in 0..=k {
        for y in 0..=k {
            let n = s - x - y;
            if 0 <= n && n <= k {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
