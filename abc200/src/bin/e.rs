use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64
    }

    let sum = (1..=n).sum();

    let mut count = 1;
    let mut s = 1;
    let mut i = 3;
    while s + if i - 1 < n { count + i - 1 } else { sum } < k {
        count = if i - 1 < n { count + i - 1 } else { sum };
        s += count;
        i += 1;
    }

    i += 1;
    let mut remains = k - s;
    for j in 1..=n {
        for k in 1..=n {
            if j + k > i - 1 {
                break;
            }
            if i - j - k > n {
                continue;
            }
            remains -= 1;
            if remains == 0 {
                println!("{} {} {}", j, k, i - j - k);
                return;
            }
        }
    }
}
