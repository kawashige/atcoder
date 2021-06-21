use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    }

    let mut count = 0;
    let mut counts = vec![0; r + 1];

    for i in (2..=r).rev() {
        let x = r / i - (l - 1) / i;
        counts[i] = x * x;
        for j in ((2 * i)..=r).step_by(i) {
            counts[i] -= counts[j];
        }
        count += counts[i];

        if 2.max(l) <= i {
            count -= 2 * x - 1;
        }
    }

    println!("{}", count);
}
