use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i32, i32); n]
    }

    ab.sort_by_cached_key(|(_, b)| -b);

    let mut t = ab[0].1 - ab[0].0;
    for i in 1..n {
        t = std::cmp::min(t, ab[i].1);
        t -= ab[i].0;
    }

    if t >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
