use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ct: [(usize, usize); n]
    }

    if let Some(r) = ct
        .into_iter()
        .filter(|(_, tt)| tt <= &t)
        .map(|(c, _)| c)
        .min()
    {
        println!("{}", r);
    } else {
        println!("TLE");
    }
}
