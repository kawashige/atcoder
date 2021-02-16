use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        spells: [(usize, usize); n]
    }

    if spells.into_iter().filter(|(x, y)| x < &s && &d < y).count() > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
