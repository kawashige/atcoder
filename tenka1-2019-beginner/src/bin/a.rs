use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if (a < b && (a..=b).contains(&c)) || (a > b && (b..=a).contains(&c)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
