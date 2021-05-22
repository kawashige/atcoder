use proconio::input;

fn main() {
    input! {
        q: usize,
        ab: [(u64, u64); q]
    }

    for (a, b) in ab {
        let mut a_count = 0;
        let mut b_count = 0;
        let mut a_rest = 0;
        let mut b_rest = 0;

        let ab = a * b - 1;
        if a != 1 {
            let mut x = a - 1;
            while x > 0 && ab / x == b {
                x -= 1;
            }

            let mut prev = x + 1;
            while x > 0 && ab as f64 / (x as f64) - ab as f64 / (prev as f64) < 1.0 {
                if ab / x > ab / prev {
                    a_count += 1;
                }
                prev = x;
                x -= 1;
            }
            a_count += x;
            a_rest = a - 1 - a_count;
        }

        if b != 1 {
            let mut x = b - 1;
            while x > 0 && ab / x == a {
                x -= 1;
            }

            let mut prev = x + 1;
            while x > 0 && ab as f64 / (x as f64) - ab as f64 / (prev as f64) < 1.0 {
                if ab / x > ab / prev {
                    b_count += 1;
                }
                prev = x;
                x -= 1;
            }
            b_count += x;
            b_rest = b - 1 - b_count;
        }

        let count = a_count + b_count + std::cmp::min(a_rest, b_rest);

        println!("{}", count);
    }
}
