use proconio::input;

fn main() {
    input! {
        q: usize,
        ab: [(u64, u64); q]
    }

    for (a, b) in ab {
        let mut a_count = 0;
        let mut b_count = 0;

        let ab = a * b - 1;
        let mut x = std::cmp::max(a - 1, b - 1);
        let mut prev = ab / (x + 1);

        while x > 0 {
            let cur = ab / x;
            let d = cur - prev;
            if d > 0 {
                if cur > b {
                    a_count += 1;
                }
                if cur > a {
                    b_count += 1;
                }
            }
            prev = cur;
            x -= 1;
            if d > 1 {
                break;
            }
        }

        // println!("a_count: {}, b_count: {}, x: {}", a_count, b_count, x);

        a_count += std::cmp::min(x, a - 1);
        b_count += std::cmp::min(x, b - 1);
        let a_rest = a - 1 - a_count;
        let b_rest = b - 1 - b_count;
        let count = a_count + b_count + std::cmp::min(a_rest, b_rest);

        println!("{}", count);
    }
}
