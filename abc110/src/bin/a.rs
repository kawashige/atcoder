use proconio::input;

fn main() {
    input! {
        mut v: [usize; 3]
    }

    v.sort_unstable();

    println!("{}", v[2] * 10 + v[1] + v[0]);
}
