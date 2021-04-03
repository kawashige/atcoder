use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut result = a.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    result.sort_unstable_by_key(|r| r.1);
    let s = result
        .into_iter()
        .map(|(i, _)| (i + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}
