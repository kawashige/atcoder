use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut r = vec![0; n];
    for i in 0..n {
        r[p[i] - 1] = i + 1;
    }

    println!(
        "{}",
        r.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
