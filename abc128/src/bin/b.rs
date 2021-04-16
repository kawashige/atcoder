use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n]
    }

    let mut sp = sp
        .into_iter()
        .zip(1..)
        .map(|((s, p), i)| (s, p, i))
        .collect::<Vec<(String, usize, usize)>>();
    sp.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    for (_, _, i) in sp {
        println!("{}", i);
    }
}
