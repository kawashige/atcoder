use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xyzw: [(usize, usize, usize, u64); q]
    }

    const M: usize = 1_000_000_007;
    let mut r = 1;

    for i in 0..60 {
        let mut count = 0;
        for j in 0..2_u64.pow(n as u32) {
            let mut invalid = false;
            for (x, y, z, w) in &xyzw {
                if ((j & 1 << (x - 1)) | (j & 1 << (y - 1)) | (j & 1 << (z - 1)) == 0)
                    != (w & 1 << i == 0)
                {
                    invalid = true;
                    break;
                }
            }
            if !invalid {
                count += 1;
            }
        }
        r *= count;
        r %= M;
    }

    println!("{}", r);
}
