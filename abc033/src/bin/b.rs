use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n]
    }

    let sum = sp.iter().map(|(_, x)| x).sum::<usize>();

    for (s, p) in sp {
        if p > sum / 2 {
            println!("{}", s);
            return;
        }
    }

    println!("atcoder");
}
