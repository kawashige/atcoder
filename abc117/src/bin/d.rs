use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n]
    }

    let mut counts = vec![0; 64];

    for x in a {
        for i in 0..64 {
            if x & 1 << i > 0 {
                counts[i] += 1;
            }
        }
    }

    let mut m = 2_u64.pow(63);
    let mut r = 0;

    let s = 64 - k.leading_zeros() as usize;
    let mut used = false;

    for i in (0_usize..64).rev() {
        if s < i {
            r += m * counts[i];
        } else {
            if k & 1 << i > 0 {
                if counts[i] > n as u64 - counts[i] {
                    r += m * counts[i];
                    used = true;
                } else {
                    r += m * (n as u64 - counts[i]);
                }
            } else {
                if counts[i] < n as u64 - counts[i] && used {
                    r += m * (n as u64 - counts[i]);
                } else {
                    r += m * counts[i];
                }
            }
        }

        m /= 2;
    }

    println!("{}", r);
}
