use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n]
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut r = 0;

    let mut count = 1;
    let mut current = a[0];

    for i in 1..n {
        if current == a[i] {
            count += 1;
            continue;
        } else if (current - a[i]) * count <= k {
            r += (current * (current + 1) / 2 - a[i] * (a[i] + 1) / 2) * count;
            k -= (current - a[i]) * count;
        } else {
            let j = k / count;
            if j > 0 {
                r += (current * (current + 1) / 2 - (current - j) * (current - j + 1) / 2) * count;
                k -= j * count;
            }
            r += (current - j) * k;
            k = 0;
        }

        current = a[i];
        count += 1;

        if k == 0 {
            println!("{}", r);
            return;
        }
    }

    if current * count <= k {
        r += (current * (current + 1) / 2) * count;
    } else {
        let j = k / count;
        if j > 0 {
            r += (current * (current + 1) / 2 - (current - j) * ((current - j) + 1) / 2) * count;
            k -= j * count;
        }
        r += (current - j) * k;
    }

    println!("{}", r);
}
