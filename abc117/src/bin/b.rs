use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n]
    }

    l.sort_unstable();

    if l[n - 1] < l[..(n - 1)].iter().sum::<usize>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
