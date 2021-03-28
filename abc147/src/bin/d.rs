use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let m = 1_000_000_007;
    let mut count_zero = [0_u64; 64];
    let mut count_one = [0_u64; 64];

    let mut sum = 0;
    for i in (0..n).rev() {
        let mut mul = 1;
        for j in 0..64 {
            if a[i] & 1 << j == 0 {
                sum = (sum + count_one[j] * mul % m) % m;
                count_zero[j] += 1;
            } else {
                sum = (sum + count_zero[j] * mul % m) % m;
                count_one[j] += 1;
            }
            mul = (mul * 2) % m;
        }
    }
    println!("{}", sum);
}
