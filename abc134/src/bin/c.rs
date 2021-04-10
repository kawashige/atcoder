use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut left = vec![0; n];
    left[0] = a[0];
    (1..n).for_each(|i| left[i] = std::cmp::max(left[i - 1], a[i]));

    let mut right = vec![0; n];
    right[n - 1] = a[n - 1];
    (0..(n - 1))
        .rev()
        .for_each(|i| right[i] = std::cmp::max(right[i + 1], a[i]));

    for i in 0..n {
        if i == 0 {
            println!("{}", right[i + 1])
        } else if i == n - 1 {
            println!("{}", left[i - 1])
        } else {
            println!("{}", std::cmp::max(left[i - 1], right[i + 1]));
        }
    }
}
