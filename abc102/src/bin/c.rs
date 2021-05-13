use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut plus = Vec::new();
    let mut minus = Vec::new();

    for (x, i) in a.iter().zip(1..) {
        if x - i < 0 {
            minus.push((x - i).abs() as i32);
        } else {
            plus.push(x - i);
        }
    }

    plus.sort_unstable_by(|a, b| b.cmp(&a));
    minus.sort_unstable_by(|a, b| b.cmp(&a));

    let mut is_minus = false;
    let mut v: Vec<i32>;
    if plus.is_empty() {
        v = minus;
        is_minus = true;
    } else if minus.is_empty() {
        v = plus;
    } else if plus[plus.len() - 1] as i64 * (2 * plus.len() as i32 - n as i32) as i64
        > minus[minus.len() - 1] as i64 * (2 * minus.len() as i32 - n as i32) as i64
    {
        v = plus;
    } else {
        v = minus;
        is_minus = true;
    }

    let mut b = 0;
    while let Some(x) = v.pop() {
        if x <= b {
            continue;
        }

        if n as i32 / 2 < v.len() as i32 + 1 {
            b = x;
        } else {
            break;
        }
    }
    if is_minus {
        b *= -1;
    }

    let r = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x - i as i32 - 1 - b).abs() as u64)
        .sum::<u64>();
    println!("{}", r);
}
