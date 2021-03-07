use proconio::input;

fn main() {
    input! {
        n: usize,
        works: [(usize, usize); n]
    }

    let mut min = std::usize::MAX;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                min = std::cmp::min(min, works[i].0 + works[i].1);
            } else {
                min = std::cmp::min(min, std::cmp::max(works[i].0, works[j].1));
            }
        }
    }

    println!("{}", min);
}
