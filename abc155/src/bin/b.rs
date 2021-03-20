use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    if a.into_iter()
        .all(|i| i % 2 == 1 || (i % 3 == 0 || i % 5 == 0))
    {
        println!("APPROVED");
    } else {
        println!("DENIED");
    }
}
