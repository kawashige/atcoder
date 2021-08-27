use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut clr: [(u64, usize, usize); m]
    }

    let mut left = vec![std::u64::MAX; n];
    let mut right = vec![std::u64::MAX; n];
    for (c, l, r) in &clr {
        left[r - 1] = left[r - 1].min(*c);
        right[l - 1] = right[l - 1].min(*c);
    }

    let mut left_c: u64 = 0;
    let mut right_c: u64 = 0;
    for i in 0..n {
        left_c = left_c.saturating_add(left[i]);
        right_c = right_c.saturating_add(right[i]);
    }

    if left_c == std::u64::MAX && right_c == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", left_c.min(right_c));
    }
}
