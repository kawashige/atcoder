use proconio::input;

fn main() {
    input! {
        n: usize,
        shops: [(usize, i32, usize); n]
    }

    if let Some(min) = shops
        .into_iter()
        .filter(|(a, _, x)| x > a)
        .min_by_key(|(_, p, _)| p.clone())
    {
        println!("{}", min.1)
    } else {
        println!("-1")
    }
}
