use proconio::input;

fn main() {
    input! {
        mut v: [usize; 3],
    }

    v.sort_unstable();
    if v[0] == 5 && v[1] == 5 && v[2] == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
