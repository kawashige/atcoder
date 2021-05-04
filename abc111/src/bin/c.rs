use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n]
    }

    let mut odd_counts = vec![0; 100_001];
    let mut even_counts = vec![0; 100_001];

    for i in 0..n {
        if i % 2 == 0 {
            even_counts[v[i]] += 1;
        } else {
            odd_counts[v[i]] += 1;
        }
    }

    let mut odds = (0..odd_counts.len())
        .map(|i| (odd_counts[i], i))
        .filter(|(c, _)| c > &0)
        .collect::<Vec<(usize, usize)>>();
    let mut evens = (0..even_counts.len())
        .map(|i| (even_counts[i], i))
        .filter(|(c, _)| c > &0)
        .collect::<Vec<(usize, usize)>>();

    odds.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    evens.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let r = if odds[0].1 == evens[0].1 {
        n - std::cmp::max(
            odds.get(1).unwrap_or(&(0, 0)).0 + evens[0].0,
            evens.get(1).unwrap_or(&(0, 0)).0 + odds[0].0,
        )
    } else {
        n - odds[0].0 - evens[0].0
    };

    println!("{}", r);
}
