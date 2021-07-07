use proconio::input;

fn main() {
    input! {
        k: u64
    }

    let mut count = 0;

    for i in 1..=((k as f64).sqrt() as u64) {
        if k % i == 0 {
            let l = k / i;
            for j in i..=((l as f64).sqrt() as u64) {
                if i > j {
                    break;
                }
                if l % j == 0 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
