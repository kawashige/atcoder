use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }

    let mut count = vec![0; n];
    for (a, b) in ab {
        count[a - 1] += 1;
        count[b - 1] += 1;
    }

    if count.iter().any(|x| *x == n - 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
