use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut a = a.into_iter().enumerate().collect::<Vec<(_, _)>>();
    a.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for (i, _) in a {
        println!("{}", i + 1);
    }
}
