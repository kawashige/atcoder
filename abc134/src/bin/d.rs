use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut result = BTreeSet::new();
    for i in (1..=n).rev() {
        let j = ((i * 2)..=n)
            .step_by(i)
            .filter(|j| result.contains(j))
            .count()
            % 2;
        if a[i - 1] != j {
            result.insert(i);
        }
    }

    println!("{}", result.len());
    if !result.is_empty() {
        println!(
            "{}",
            result
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
