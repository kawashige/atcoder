use proconio::input;

fn main() {
    input! {
        mut a: [usize; 3]
    }

    a.sort_unstable();

    if a[1] - a[0] == a[2] - a[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
