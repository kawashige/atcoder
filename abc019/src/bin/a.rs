use proconio::input;

fn main() {
    input! {
        mut v: [usize; 3]
    }

    v.sort_unstable();
    println!("{}", v[1]);
}
