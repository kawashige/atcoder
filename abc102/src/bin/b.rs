use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut max = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            max = std::cmp::max(max, (a[i] - a[j]).abs());
        }
    }

    println!("{}", max);
}
