use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    let mut count: i64 = 0;

    for i in 0..(n - 1) {
        if a[i] < b[i] {
            let x = (b[i] - a[i] + 1) / 2;
            a[i] += 2 * x;
            count += x as i64;
        }

        if a[i] > b[i] {
            let x = a[i] - b[i];
            b[i] += x;
            count -= x as i64;
        }
    }

    if count > 0 {
        b[n - 1] += count as usize;
    } else {
        a[n - 1] += (count.abs() * 2) as usize;
    }

    println!("{}", if a[n - 1] <= b[n - 1] { "Yes" } else { "No" });
}
