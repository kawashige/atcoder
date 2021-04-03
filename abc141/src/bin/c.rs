use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; q]
    }

    let mut points = vec![k; n];
    for i in 0..q {
        points[a[i] - 1] += 1;
    }
    for i in 0..n {
        if points[i] > q {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
