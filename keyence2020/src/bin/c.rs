use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: usize
    }

    let r = std::iter::repeat(s.to_string())
        .take(k)
        .chain(
            std::iter::repeat(
                if s == 1_000_000_000 {
                    "1"
                } else {
                    "1000000000"
                }
                .to_string(),
            )
            .take(n - k),
        )
        .collect::<Vec<_>>();

    println!("{}", r.join(" "));
}
