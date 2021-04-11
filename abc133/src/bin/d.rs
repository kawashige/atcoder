use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut num = a[0];
    for i in 1..n {
        num = a[i] - num;
    }

    let mut b = vec![num];
    for i in 1..n {
        b.push(2 * a[i - 1] - b[i - 1])
    }

    println!(
        "{}",
        b.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
