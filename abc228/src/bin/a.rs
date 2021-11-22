use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize
    }

    if (s < t && (s..=t).contains(&x)) || (s > t && !(t..s).contains(&x)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
